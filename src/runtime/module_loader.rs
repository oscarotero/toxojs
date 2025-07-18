use std::path::PathBuf;

use data_url::DataUrl;
use deno_ast::ModuleSpecifier;
use deno_core::{
    ModuleLoadResponse, ModuleLoader, ModuleSource, ModuleSourceCode, ModuleType,
    RequestedModuleType, error::ModuleLoaderError, futures::FutureExt, resolve_import, url::Url,
};
use deno_error::JsErrorBox;
use deno_fetch::{Client, ReqBody, create_http_client};
use http::Request;
use http_body_util::BodyExt;
use import_map::{ImportMap, parse_from_json};

pub struct ToxoModuleLoaderOptions {
    pub main_module: Url,
    pub user_agent: String,
    pub vendor_directory: Option<PathBuf>,
}

pub struct ToxoModuleLoader {
    client: Client,
    import_map: Option<ImportMap>,
    main_module: Url,
    vendor_directory: Option<PathBuf>,
}

impl ToxoModuleLoader {
    pub fn new(options: ToxoModuleLoaderOptions) -> ToxoModuleLoader {
        let client = create_http_client(&options.user_agent, Default::default()).unwrap();
        let main_module = options.main_module;

        // Detect the import map only if it's in the filesystem
        let import_map = if main_module.scheme() == "file" {
            let import_map = main_module.join("./import_map.json").unwrap();
            let file_path = import_map.to_file_path().unwrap();
            if file_path.exists() {
                Some(import_map)
            } else {
                None
            }
        } else {
            None
        };

        // If the import map was detected, load and parse it
        let import_map: Option<ImportMap> = if let Some(url) = import_map {
            let content = read_file(&url).unwrap();
            let content = str::from_utf8(&content).unwrap();
            let map = parse_from_json(url, content).unwrap();

            Some(map.import_map)
        } else {
            None
        };

        ToxoModuleLoader {
            client,
            import_map,
            main_module,
            vendor_directory: options.vendor_directory,
        }
    }
}

impl ModuleLoader for ToxoModuleLoader {
    fn resolve(
        &self,
        raw_specifier: &str,
        raw_referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<ModuleSpecifier, ModuleLoaderError> {
        let import_map = &self.import_map;

        // Use the import_map to resove the module
        if let Some(import_map) = import_map {
            let referrer = if raw_referrer == "." {
                &self.main_module
            } else {
                &Url::parse(raw_referrer).unwrap()
            };

            if let Ok(specifier) = import_map.resolve(raw_specifier, &referrer) {
                return Ok(specifier);
            }
        }

        // Use default import
        resolve_import(raw_specifier, raw_referrer).map_err(|e| e.into())
    }

    fn load(
        &self,
        specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dynamic: bool,
        requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let specifier = specifier.clone();
        let client = self.client.clone();
        let vendor_directory = self.vendor_directory.clone();

        let fut = async move {
            // Calculate the module type
            let module_type = match &requested_module_type {
                RequestedModuleType::Text => ModuleType::Text,
                RequestedModuleType::Bytes => ModuleType::Bytes,
                RequestedModuleType::Json => {
                    if !has_extension(&specifier, "json") {
                        return Err(ModuleLoaderError::JsonMissingAttribute);
                    }
                    ModuleType::Json
                }
                RequestedModuleType::Other(_ty) => {
                    return Err(ModuleLoaderError::Unsupported {
                        specifier: Box::new(specifier),
                        maybe_referrer: None,
                    });
                }
                _ => {
                    if has_extension(&specifier, "wasm") {
                        ModuleType::Wasm
                    } else if has_extension(&specifier, "json") {
                        return Err(ModuleLoaderError::JsonMissingAttribute);
                    } else {
                        ModuleType::JavaScript
                    }
                }
            };

            // Load the module
            let code = if specifier.scheme() == "file" {
                read_file(&specifier)
            } else if specifier.scheme() == "data" {
                read_data(&specifier, &module_type)
            } else {
                if let Some(vendor_directory) = vendor_directory {
                    read_vendor_url(&specifier, &client, &vendor_directory).await
                } else {
                    read_url(&specifier, &client).await
                }
            };

            // Return the code if it's found or an error
            match code {
                Ok(code) => {
                    let module = ModuleSource::new(
                        module_type,
                        ModuleSourceCode::Bytes(code.into_boxed_slice().into()),
                        &specifier,
                        None,
                    );
                    Ok(module)
                }
                Err(err) => Err(ModuleLoaderError::from(err)),
            }
        }
        .boxed_local();

        ModuleLoadResponse::Async(fut)
    }
}

/** Read the code from the file system */
fn read_file(specifier: &Url) -> Result<Vec<u8>, JsErrorBox> {
    let path = specifier
        .to_file_path()
        .map_err(|_| JsErrorBox::generic(format!("\"{specifier}\" is not a valid file URL.")))?;

    std::fs::read(path).map_err(|source| JsErrorBox::from_err(source))
}

/** Parse data urls */
fn read_data(specifier: &Url, module_type: &ModuleType) -> Result<Vec<u8>, JsErrorBox> {
    let url = DataUrl::process(specifier.as_str());
    match url {
        Ok(url) => {
            let (body, _) = url.decode_to_vec().unwrap();
            let mime = url.mime_type();
            let mime = format!("{}/{}", mime.type_, mime.subtype).to_lowercase();
            match module_type {
                ModuleType::JavaScript => {
                    if mime == "application/javascript" || mime == "text/javascript" {
                        Ok(body)
                    } else {
                        Err(JsErrorBox::generic("Invalid mime type of data URL"))
                    }
                }
                ModuleType::Json => {
                    if mime == "application/json" {
                        Ok(body)
                    } else {
                        Err(JsErrorBox::generic("Invalid mime type of data URL"))
                    }
                }
                _ => Ok(body),
            }
        }
        Err(_) => Err(JsErrorBox::generic("Unable to load data: specifier")),
    }
}

/** Read the code from an Url */
async fn read_url(specifier: &Url, client: &Client) -> Result<Vec<u8>, JsErrorBox> {
    let body = ReqBody::empty();
    let mut request = Request::new(body);
    *request.uri_mut() = http::Uri::try_from(specifier.as_str()).unwrap();
    let response = match client.clone().send(request).await {
        Ok(response) => response,
        Err(err) => return Err(JsErrorBox::from_err(err)),
    };

    if response.status() == 404 {
        return Err(JsErrorBox::generic(format!("Module {specifier} not found")));
    }
    if !response.status().is_success() {
        let status = response.status();

        return Err(JsErrorBox::generic(format!(
            "Cannot fetch {specifier}. Served returned {status} code."
        )));
    }

    let body = response.into_body().collect().await;
    let bytes = body.unwrap().to_bytes();
    let code: Vec<u8> = bytes.into();
    Ok(code)
}

/** Read the code from the vendor code, or download it if it wasn't cached yet */
async fn read_vendor_url(
    specifier: &Url,
    client: &Client,
    vendor_directory: &PathBuf,
) -> Result<Vec<u8>, JsErrorBox> {
    let vendor_path = format!("{}{}", specifier.host().unwrap(), specifier.path());
    let vendor_path = vendor_directory.join(vendor_path);

    if vendor_path.exists() {
        std::fs::read(vendor_path).map_err(|source| JsErrorBox::from_err(source))
    } else {
        let result = read_url(specifier, client).await;
        if let Ok(ref contents) = result {
            if let Some(parent) = vendor_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&vendor_path, contents);
        }
        result
    }
}

fn has_extension(url: &Url, extension: &str) -> bool {
    let path = PathBuf::from(url.path());
    if let Some(ext) = path.extension() {
        return ext.to_string_lossy().to_lowercase() == extension;
    }
    false
}

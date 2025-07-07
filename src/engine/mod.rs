use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::error::AnyError;
use deno_core::extension;
use deno_core::op2;
use deno_core::url::Url;
use deno_permissions::PermissionsContainer;
use deno_tls::rustls;
use std::env;
use std::env::current_dir;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crate::engine::permissions::RuntimePermissionDescriptorParser;

pub mod module_loader;
pub mod permissions;
pub mod transpile;

pub struct Engine {
    runtime: JsRuntime,
    main_module: Url,
    main_directory: PathBuf,
}

extension!(
    toxo_setup,
    ops = [
        op_toxo_languages,
        op_toxo_user_agent
    ],
    esm_entry_point = "ext:toxo_setup/bootstrap.js",
    esm = [dir "src/js", "bootstrap.js"]
);

#[op2]
#[string]
pub fn op_toxo_languages() -> String {
    match env::var("TOXO_LANGUAGES") {
        Ok(languages) => languages,
        Err(_) => String::from("en-US"),
    }
}

#[op2]
#[string]
pub fn op_toxo_user_agent() -> String {
    get_user_agent()
}

pub mod sys {
    #[allow(clippy::disallowed_types)] // ok, definition
    pub type CliSys = sys_traits::impls::RealSys;
}

impl Engine {
    pub fn new(main_module: Url) -> Engine {
        // Calculate the current directory
        let main_directory = if main_module.scheme() == "file" {
            main_module
                .to_file_path()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf()
        } else {
            current_dir().unwrap()
        };

        // Init the engine extensions to provide Web APIs
        let parser = RuntimePermissionDescriptorParser::new(main_directory.clone());
        let permissions = PermissionsContainer::allow_all(Arc::new(parser));

        let extensions: Vec<Extension> = vec![
            deno_telemetry::deno_telemetry::init(),
            deno_webidl::deno_webidl::init(),
            deno_console::deno_console::init(),
            deno_url::deno_url::init(),
            deno_web::deno_web::lazy_init::<PermissionsContainer>(),
            deno_webgpu::deno_webgpu::init(),
            deno_fetch::deno_fetch::lazy_init::<PermissionsContainer>(),
            deno_webstorage::deno_webstorage::lazy_init(),
            deno_crypto::deno_crypto::lazy_init(),
            deno_net::deno_net::lazy_init::<PermissionsContainer>(),
            deno_tls::deno_tls::init(),
            toxo_setup::init(),
        ];

        // This is required by some net related ops
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .unwrap();

        // Initialize the module loader
        let user_agent = get_user_agent();
        let module_loader = module_loader::ToxoModuleLoader::new(main_module.clone(), &user_agent);

        // Create the JavaScript runtime
        let runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(module_loader)),
            extensions,
            extension_transpiler: Some(Rc::new(|specifier, source| {
                transpile::maybe_transpile_source(specifier, source)
            })),
            ..Default::default()
        });

        // Store the permissions in the runtime state
        let state = runtime.op_state();
        let mut state = state.borrow_mut();

        state.put::<PermissionsContainer>(permissions);

        Engine {
            runtime,
            main_module,
            main_directory,
        }
    }

    /** Run the JavaScript code */
    pub async fn run(&mut self) -> Result<(), AnyError> {
        let runtime = &mut self.runtime;
        let specifier = &self.main_module;
        let main_directory = &self.main_directory;

        //Initialize the extensions configured as lazy_init
        runtime
            .lazy_init_extensions(vec![
                deno_web::deno_web::args::<PermissionsContainer>(
                    Default::default(),
                    Some(specifier.clone()),
                ),
                deno_fetch::deno_fetch::args::<PermissionsContainer>(deno_fetch::Options {
                    ..Default::default()
                }),
                deno_webstorage::deno_webstorage::args(Some(main_directory.clone())),
                deno_crypto::deno_crypto::args(Default::default()),
                deno_net::deno_net::args::<PermissionsContainer>(
                    Default::default(),
                    Default::default(),
                ),
            ])
            .unwrap();

        // Run the module
        let main_id = runtime.load_main_es_module(&specifier).await?;
        let main_result = runtime.mod_evaluate(main_id);
        runtime.run_event_loop(Default::default()).await?;
        main_result.await.map_err(AnyError::from)
    }
}

fn get_user_agent() -> String {
    match env::var("TOXO_USER_AGENT") {
        Ok(languages) => languages,
        Err(_) => {
            let version = env!("CARGO_PKG_VERSION");
            let user_agent = format!("TOXO/{}", version);
            user_agent
        }
    }
}

use deno_cache::CacheImpl;
use deno_cache::CreateCache;
use deno_cache::SqliteBackedCache;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::error::AnyError;
use deno_core::url::Url;
use deno_fs::FileSystemRc;
use deno_fs::RealFs;
use deno_io::Stdio;
use deno_permissions::PermissionsContainer;
use deno_tls::rustls;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crate::ops::bootstrap;
use crate::ops::deno_tty;
use crate::ops::environment;
use crate::ops::environment::GlobalVars;
use crate::ops::filesystem;
use crate::ops::navigator;
use crate::ops::prompt;
use crate::runtime::module_loader::ToxoModuleLoader;
use crate::runtime::module_loader::ToxoModuleLoaderOptions;
use crate::runtime::permissions::RuntimePermissionDescriptorParser;
use crate::runtime::transpile;

pub struct MainWorker {
    pub runtime: JsRuntime,
    pub main_module: Url,
    pub vendor_directory: Option<PathBuf>,
    pub storage_directory: PathBuf,
    pub user_agent: String,
    pub languages: String,
}

pub mod sys {
    #[allow(clippy::disallowed_types)] // ok, definition
    pub type CliSys = sys_traits::impls::RealSys;
}

pub struct MainWorkerOptions {
    pub main_module: Url,
    pub vendor_directory: Option<PathBuf>,
    pub storage_directory: PathBuf,
    pub user_agent: String,
    pub languages: String,
}

/** The initial worker to run TOXO */
impl MainWorker {
    pub fn new(options: MainWorkerOptions) -> MainWorker {
        let main_module = options.main_module;
        let vendor_directory = options.vendor_directory;
        let storage_directory = options.storage_directory;

        // Init the extensions to provide Web APIs
        let parser = RuntimePermissionDescriptorParser::new(
            storage_directory.parent().unwrap().to_path_buf(),
        );
        let permissions = PermissionsContainer::allow_all(Arc::new(parser));
        let globals = GlobalVars {
            location: main_module.to_string(),
            user_agent: options.user_agent.clone(),
            languages: options.languages.clone(),
        };

        let extensions: Vec<Extension> = vec![
            // Deno extensions
            deno_telemetry::deno_telemetry::init(),
            deno_webidl::deno_webidl::init(),
            deno_console::deno_console::init(),
            deno_url::deno_url::init(),
            deno_web::deno_web::lazy_init::<PermissionsContainer>(),
            deno_webgpu::deno_webgpu::init(),
            deno_fetch::deno_fetch::lazy_init::<PermissionsContainer>(),
            deno_cache::deno_cache::lazy_init(),
            deno_websocket::deno_websocket::lazy_init::<PermissionsContainer>(),
            deno_webstorage::deno_webstorage::lazy_init(),
            deno_crypto::deno_crypto::lazy_init(),
            deno_net::deno_net::lazy_init::<PermissionsContainer>(),
            deno_tls::deno_tls::init(),
            deno_io::deno_io::lazy_init(),
            // Deno runtime ops
            deno_tty::deno_tty::init(),
            deno_fs::deno_fs::lazy_init::<PermissionsContainer>(),
            // Toxo custom extensions
            environment::toxo_env::init(globals),
            navigator::toxo_navigator::init(),
            prompt::toxo_prompt::init(),
            bootstrap::toxo_setup::init(),
            filesystem::toxo_filesystem::init(),
        ];

        // This is required by some net related ops
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .unwrap();

        // Initialize the module loader
        let module_loader = ToxoModuleLoader::new(ToxoModuleLoaderOptions {
            main_module: main_module.clone(),
            user_agent: options.user_agent.clone(),
            vendor_directory: vendor_directory.clone(),
        });

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

        MainWorker {
            runtime,
            main_module,
            storage_directory,
            vendor_directory,
            user_agent: options.user_agent,
            languages: options.languages,
        }
    }

    /** Run the JavaScript code */
    pub async fn run(&mut self) -> Result<(), AnyError> {
        let runtime = &mut self.runtime;
        let specifier = &self.main_module;
        let storage_directory = self.storage_directory.clone();
        let cache_directory = storage_directory.clone();
        let fs: FileSystemRc = Rc::new(RealFs);

        let create_cache_fn = move || {
            let sqlite = SqliteBackedCache::new(cache_directory.join("caches"))?;
            Ok(CacheImpl::Sqlite(sqlite))
        };

        let cache: Option<CreateCache> = Some(CreateCache(Arc::new(create_cache_fn)));

        //Initialize the extensions configured as lazy_init
        runtime
            .lazy_init_extensions(vec![
                deno_web::deno_web::args::<PermissionsContainer>(
                    Default::default(),
                    Some(specifier.clone()),
                ),
                deno_fetch::deno_fetch::args::<PermissionsContainer>(deno_fetch::Options {
                    user_agent: self.user_agent.clone(),
                    ..Default::default()
                }),
                deno_cache::deno_cache::args(cache),
                deno_websocket::deno_websocket::args::<PermissionsContainer>(
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ),
                deno_webstorage::deno_webstorage::args(Some(storage_directory.clone())),
                deno_crypto::deno_crypto::args(Default::default()),
                deno_net::deno_net::args::<PermissionsContainer>(
                    Default::default(),
                    Default::default(),
                ),
                deno_io::deno_io::args(Some(Stdio::default())),
                deno_fs::deno_fs::args::<PermissionsContainer>(fs),
            ])
            .unwrap();

        // Run the module
        let main_id = runtime.load_main_es_module(&specifier).await?;
        let main_result = runtime.mod_evaluate(main_id);
        runtime.run_event_loop(Default::default()).await?;
        main_result.await.map_err(AnyError::from)
    }
}

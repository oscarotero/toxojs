use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::error::AnyError;
use deno_core::extension;
use deno_core::resolve_url_or_path;
use deno_permissions::PermissionsContainer;
use deno_tls::rustls;
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
    initial_cwd: PathBuf,
}

extension!(
    toxo_setup,
    esm_entry_point = "ext:toxo_setup/bootstrap.js",
    esm = [dir "src/engine", "bootstrap.js"]
);

pub mod sys {
    #[allow(clippy::disallowed_types)] // ok, definition
    pub type CliSys = sys_traits::impls::RealSys;
}

impl Engine {
    pub fn new() -> Engine {
        let parser = RuntimePermissionDescriptorParser::new();
        let permissions = PermissionsContainer::allow_all(Arc::new(parser));

        let extensions: Vec<Extension> = vec![
            deno_telemetry::deno_telemetry::init(),
            deno_webidl::deno_webidl::init(),
            deno_console::deno_console::init(),
            deno_url::deno_url::init(),
            deno_web::deno_web::lazy_init::<PermissionsContainer>(),
            deno_fetch::deno_fetch::lazy_init::<PermissionsContainer>(),
            deno_webstorage::deno_webstorage::lazy_init(),
            deno_crypto::deno_crypto::lazy_init(),
            deno_net::deno_net::lazy_init::<PermissionsContainer>(),
            deno_tls::deno_tls::init(),
            toxo_setup::init(),
        ];
        let initial_cwd = current_dir().unwrap();
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .unwrap();
        // let module_loader = deno_core::FsModuleLoader;
        let module_loader = module_loader::ToxoModuleLoader::new();
        let runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(module_loader)),
            extensions,
            extension_transpiler: Some(Rc::new(|specifier, source| {
                transpile::maybe_transpile_source(specifier, source)
            })),
            ..Default::default()
        });
        let state = runtime.op_state();
        let mut state = state.borrow_mut();

        state.put::<PermissionsContainer>(permissions);

        Engine {
            runtime,
            initial_cwd,
        }
    }

    pub async fn run_main(&mut self, file_path: &str) -> Result<(), AnyError> {
        let runtime = &mut self.runtime;
        let initial_cwd = &self.initial_cwd;
        let specifier = resolve_url_or_path(file_path, initial_cwd)?;

        runtime
            .lazy_init_extensions(vec![
                deno_web::deno_web::args::<PermissionsContainer>(
                    Default::default(),
                    Some(specifier.clone()),
                ),
                deno_fetch::deno_fetch::args::<PermissionsContainer>(deno_fetch::Options {
                    ..Default::default()
                }),
                deno_webstorage::deno_webstorage::args(Some(initial_cwd.clone())),
                deno_crypto::deno_crypto::args(Default::default()),
                deno_net::deno_net::args::<PermissionsContainer>(
                    Default::default(),
                    Default::default(),
                ),
            ])
            .unwrap();

        let main_id = runtime.load_main_es_module(&specifier).await?;
        let main_result = runtime.mod_evaluate(main_id);
        runtime.run_event_loop(Default::default()).await?;
        main_result.await.map_err(AnyError::from)
    }
}

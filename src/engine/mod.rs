use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::error::AnyError;
use deno_core::extension;
use deno_core::resolve_path;
use deno_permissions::PermissionsContainer;
use std::env::current_dir;
use std::rc::Rc;

pub struct Engine {
    runtime: JsRuntime,
}

extension!(
    toxo_setup,
    esm_entry_point = "ext:toxo_setup/bootstrap.js",
    esm = [dir "src/engine", "bootstrap.js"]
);

impl Engine {
    pub fn new() -> Engine {
        let location = resolve_path(".", &current_dir().unwrap()).unwrap();
        let extensions: Vec<Extension> = vec![
            deno_webidl::deno_webidl::init(),
            deno_console::deno_console::init(),
            deno_url::deno_url::init(),
            deno_web::deno_web::init::<PermissionsContainer>(Default::default(), Some(location)),
            deno_net::deno_net::init::<PermissionsContainer>(None, None),
            // deno_fetch::deno_fetch::init::<PermissionsContainer>(Default::default()),
            toxo_setup::init(),
        ];

        let runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            extensions,
            ..Default::default()
        });

        Engine { runtime }
    }

    pub async fn run_main(&mut self, file_path: &str) -> Result<(), AnyError> {
        let runtime = &mut self.runtime;

        let current_dir = current_dir()?;
        let specifier = resolve_path(file_path, &current_dir)?;
        let main_id = runtime.load_main_es_module(&specifier).await?;
        let main_result = runtime.mod_evaluate(main_id);
        runtime.run_event_loop(Default::default()).await?;
        main_result.await.map_err(AnyError::from)
    }
}

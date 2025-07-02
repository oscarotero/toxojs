use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::ModuleSpecifier;
use deno_core::RuntimeOptions;
use deno_core::error::AnyError;
use deno_core::resolve_path;
use std::env::current_dir;
use std::rc::Rc;

pub struct Engine {
    runtime: JsRuntime,
}

impl Engine {
    pub fn new() -> Engine {
        // let extensions: Vec<Extension> = vec![
        //   deno_console::deno_console::init()
        // ];

        let runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            // extensions,
            ..Default::default()
        });

        Engine { runtime }
    }

    pub async fn run_main(&mut self, file_path: &str) -> Result<(), AnyError> {
        let runtime = &mut self.runtime;

        // Load Web APIs
        // let specifier = ModuleSpecifier::parse("toxo:web.js")?;
        // let code = include_str!("web.js");
        // let web_id = runtime.load_side_es_module_from_code(&specifier, code).await?;
        // let web_result = runtime.mod_evaluate(web_id);

        let current_dir = current_dir()?;
        let specifier = resolve_path(file_path, &current_dir)?;
        let main_id = runtime.load_main_es_module(&specifier).await?;
        let main_result = runtime.mod_evaluate(main_id);
        runtime.run_event_loop(Default::default()).await?;
        // web_result.await?;
        main_result.await.map_err(AnyError::from)
    }
}

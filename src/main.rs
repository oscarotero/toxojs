use deno_core::resolve_url_or_path;
use std::env::{self, current_dir};
use tokio::runtime::Builder;
use toxo::engine::Engine;

fn main() {
    // Get the first argument or show help()
    let args: Vec<String> = env::args().collect();
    let main_module = match args.get(1) {
        Some(path) => path,
        None => {
            help();
            return;
        }
    };

    // Convert the first argument to Url
    let main_module = resolve_url_or_path(&main_module, &current_dir().unwrap()).unwrap();

    // Create and run the JavaScript engine
    let mut engine = Engine::new(main_module);
    let result = engine.run();
    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    if let Err(error) = runtime.block_on(result) {
        eprintln!("error: {}", error);
    }
}

/** Help to show if no arguments were passed */
fn help() {
    let version = env!("CARGO_PKG_VERSION");
    println!("TOXO {}", version);
    println!("");
    println!("Run: toxo <file>");
    println!("Example: toxo script.js");
    println!("");
    println!(
        "Tip: Place an import_map.json file in your current directory to automatically enable import maps."
    );
}

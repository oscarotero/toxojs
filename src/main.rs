use std::env::{self, current_dir};
use tokio::runtime::Builder;
use toxo::engine::Engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            help();
            return;
        }
    };

    let initial_cwd = current_dir().unwrap();
    let mut engine = Engine::new(initial_cwd);
    let result = engine.run_main(file_path);
    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    if let Err(error) = runtime.block_on(result) {
        eprintln!("error: {}", error);
    }
}

fn help() {
    let version = env!("CARGO_PKG_VERSION");
    println!("TOXO {}", version);
    println!("");
    println!("Run: toxo <file>");
    println!("Example: toxo script.js");
    println!("");
    println!("Tip: Place an import_map.json file in your current directory to automatically enable import maps.");
}
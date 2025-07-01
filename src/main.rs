use std::env;
use toxo::engine::{loader::Loader, v8::Engine};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = match args.get(1) {
        Some(file) => file,
        None => {
            println!("No file specified!");
            return;
        }
    };

    let loader = Loader::new();
    let content = loader.load(file);
    let engine = Engine::new();

    if let Ok(result) = engine.run(&content) {
        println!("{}", result);
    }
}

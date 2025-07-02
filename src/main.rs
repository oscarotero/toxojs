use std::env;
use tokio::runtime::Builder;
use toxo::engine::Engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            println!("No file specified!");
            return;
        }
    };

    let mut engine = Engine::new();
    let result = engine.run_main(file_path);
    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    if let Err(error) = runtime.block_on(result) {
        eprintln!("error: {}", error);
    }
}

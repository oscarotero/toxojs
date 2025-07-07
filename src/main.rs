use deno_core::resolve_url_or_path;
use dotenvy::from_filename;
use std::{
    env::{self, current_dir},
    path::PathBuf,
};
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

    if main_module.scheme() == "file" {
        let env_file = main_module.join(".env").unwrap();
        let env_file = env_file.to_file_path().unwrap();
        if env_file.exists() {
            load_env_variables(env_file);
        }
    }

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

fn load_env_variables(path: PathBuf) {
    match from_filename(path) {
        Ok(_) => {}
        Err(error) => match error {
            dotenvy::Error::LineParse(line, index) => eprintln!(
                ".env file parsing error at index: {} of the value: {}",
                index, line
            ),
            dotenvy::Error::Io(_) => eprintln!(".env file was not found."),
            dotenvy::Error::EnvVar(_) => eprintln!(
                "One or more of the environment variables isn't present or not unicode within the .env file"
            ),
            _ => eprintln!("Unknown failure occurred with the .env file"),
        },
    }
}

use deno_core::{resolve_url_or_path, url::Url};
use dotenvy::from_filename;
use std::{
    env::{self, current_dir},
    path::PathBuf,
};
use tokio::runtime::Builder;
use toxo::runtime::main_worker::{MainWorker, MainWorkerOptions};

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

    let cwd = current_dir().unwrap();

    // Convert the first argument to Url
    let main_module = resolve_url_or_path(&main_module, &cwd).unwrap();

    // Load the environment variables
    if let Some(env_file) = resolve_local_path(".env", &main_module) {
        if env_file.exists() {
            load_env_variables(env_file);
        }
    }

    // Define the vendor directory
    let vendor_directory = if let Ok(path) = env::var("TOXO_VENDOR") {
        if path.to_lowercase() == "none" {
            None
        } else {
            Some(path)
        }
    } else {
        Some(String::from("vendor"))
    };

    // Define the storage folder
    let storage_directory = if let Some(path) = resolve_local_path("storage", &main_module) {
        path
    } else {
        cwd.join("storage")
    };

    let vendor_directory =
        vendor_directory.and_then(|folder| resolve_local_path(&folder, &main_module));

    // Create and run the JavaScript engine
    let mut engine = MainWorker::new(MainWorkerOptions {
        main_module,
        vendor_directory,
        storage_directory,
        languages: get_languages(),
        user_agent: get_user_agent(),
    });
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

fn resolve_local_path(path: &str, main_module: &Url) -> Option<PathBuf> {
    if main_module.scheme() == "file" {
        let resolved = main_module.join(path).unwrap();
        Some(resolved.to_file_path().unwrap())
    } else {
        None
    }
}

fn get_languages() -> String {
    match env::var("TOXO_LANGUAGES") {
        Ok(languages) => languages,
        Err(_) => String::from("en-US"),
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

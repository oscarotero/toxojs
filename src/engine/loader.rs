use std::fs;

pub struct Loader {}

impl Loader {
    pub fn new() -> Loader {
        Loader {}
    }

    pub fn load(&self, specifier: &str) -> String {
        fs::read_to_string(specifier).expect("Failed to read file")
    }
}

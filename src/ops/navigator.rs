use deno_core::extension;
use std::env;

extension!(
    toxo_navigator,
    esm = [dir "src/ops", "navigator.js"]
);

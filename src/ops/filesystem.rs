use deno_core::extension;
use std::env;

extension!(
    toxo_filesystem,
    esm = [dir "src/ops", "filesystem.js"]
);

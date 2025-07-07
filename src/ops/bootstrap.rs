use deno_core::extension;
use std::env;

extension!(
    toxo_setup,
    esm_entry_point = "ext:toxo_setup/bootstrap.js",
    esm = [dir "src/ops", "bootstrap.js"]
);

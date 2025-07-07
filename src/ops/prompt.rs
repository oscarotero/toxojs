use deno_core::extension;
use std::env;

extension!(
    toxo_prompt,
    esm = [dir "src/ops", "prompt.js"]
);

use deno_core::extension;
use deno_core::op2;
use std::env;

extension!(
    toxo_navigator,
    ops = [
        op_toxo_languages,
        op_toxo_user_agent
    ],
    esm = [dir "src/ops", "navigator.js"]
);

#[op2]
#[string]
pub fn op_toxo_languages() -> String {
    match env::var("TOXO_LANGUAGES") {
        Ok(languages) => languages,
        Err(_) => String::from("en-US"),
    }
}

#[op2]
#[string]
pub fn op_toxo_user_agent() -> String {
    get_user_agent()
}

pub fn get_user_agent() -> String {
    match env::var("TOXO_USER_AGENT") {
        Ok(languages) => languages,
        Err(_) => {
            let version = env!("CARGO_PKG_VERSION");
            let user_agent = format!("TOXO/{}", version);
            user_agent
        }
    }
}

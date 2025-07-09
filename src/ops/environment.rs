use deno_core::OpState;
use deno_core::extension;
use deno_core::op2;

pub struct GlobalVars {
    pub location: String,
    pub user_agent: String,
    pub languages: String,
}

extension!(
    toxo_env,
    ops = [
        op_get_location,
        op_get_user_agent,
        op_get_languages,
    ],
    options = {
      env: GlobalVars,
    },
    state = |state, options| {
        state.put(options.env);
    },
);

#[op2]
#[string]
fn op_get_location(state: &mut OpState) -> String {
    state.borrow::<GlobalVars>().location.clone()
}

#[op2]
#[string]
fn op_get_user_agent(state: &mut OpState) -> String {
    state.borrow::<GlobalVars>().user_agent.clone()
}

#[op2]
#[string]
fn op_get_languages(state: &mut OpState) -> String {
    state.borrow::<GlobalVars>().languages.clone()
}

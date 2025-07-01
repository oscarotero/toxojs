use v8;

pub struct Engine;

impl Engine {
    // Initialize V8
    pub fn new() -> Engine {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform.clone());
        v8::V8::initialize();
        Engine
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            v8::V8::dispose();
        }
        v8::V8::dispose_platform();
    }
}

impl Engine {
    pub fn run(&self, source: &str) -> Result<String, ()> {
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        // Create a new context.
        let context = v8::Context::new(handle_scope, Default::default());

        // Enter the context for compiling and running the script.
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Create a string containing the JavaScript source code.
        let code = v8::String::new(scope, source).unwrap();

        // Compile the source code.
        let script = match v8::Script::compile(scope, code, None) {
            Some(script) => script,
            None => return Err(()),
        };

        // Run the script to get the result.
        let result = script.run(scope).unwrap();

        // Convert the result to a string and print it.
        let result = result.to_string(scope).unwrap();
        let result = result.to_rust_string_lossy(scope);
        Ok(result)
    }
}

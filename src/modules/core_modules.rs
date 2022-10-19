use std::collections::HashMap;
use crate::JSRunner;
use crate::modules::Module;

pub fn command_module() -> Module {
    Module {
        name: "Command".to_string(),
        objects: vec![],
        functions: HashMap::from(
            [("print", v8::MapFnTo::map_fn_to(print))]
        )
    }
}

fn print(scope: &mut v8::HandleScope,
            args: v8::FunctionCallbackArguments,
            return_value: v8::ReturnValue) {
    if args.length() != 1 {
        let message = v8::String::new(scope, "Incorrect arguments".as_ref()).unwrap();
        let exception = v8::Exception::syntax_error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    JSRunner::log(scope, args.get(0).to_rust_string_lossy(scope).as_str())
}
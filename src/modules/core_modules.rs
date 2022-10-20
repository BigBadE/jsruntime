use std::cell::RefCell;
use std::collections::HashMap;
use crate::externalfunctions::get_string;
use crate::JSRunner;
use crate::modules::Module;

pub fn command_module() -> Module {
    Module {
        name: "Command".to_string(),
        objects: vec![],
        functions: HashMap::from(
            [("print", v8::MapFnTo::map_fn_to(print)),
                ("get_command", v8::MapFnTo::map_fn_to(get_command))]
        )
    }
}

fn get_command(scope: &mut v8::HandleScope,
               _args: v8::FunctionCallbackArguments,
               mut return_value: v8::ReturnValue) {
    let state = JSRunner::get_state(scope);
    let state = RefCell::borrow_mut(&state);
    let function: fn(i32) -> (*const u16, i32) = unsafe { std::mem::transmute(state.external_functions["get_command"]) };
    let output = function(state.id);
    let input = match get_string(output.0, output.1 as u16) {
        Ok(result) => result,
        Err(error) => error.to_string()
    };
    return_value.set(v8::Local::from(v8::String::new(scope, input.as_str()).unwrap()));
}

fn print(scope: &mut v8::HandleScope,
            args: v8::FunctionCallbackArguments,
            _return_value: v8::ReturnValue) {
    if args.length() != 1 {
        let message = v8::String::new(scope, "Incorrect arguments".as_ref()).unwrap();
        let exception = v8::Exception::syntax_error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    let message = args.get(0).to_rust_string_lossy(scope);
    JSRunner::log(&scope, message.as_str());
}
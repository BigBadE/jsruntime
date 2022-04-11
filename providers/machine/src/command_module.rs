use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use runner::imports::Provider;
use runner::state::JSRunnerState;
use util::error::JsError;
use util::fmt_error::PrettyJsError;

pub fn command_provider() -> Provider {
    Provider {
        module: Option::Some("Command"),
        functions: Option::None,
        objects: Option::Some(HashMap::from([("system", HashMap::from(
            [("run_commands", v8::MapFnTo::map_fn_to(run_cmd))]
        ))])),
    }
}

fn run_cmd<'s>(scope: &mut v8::HandleScope<'s>,
               args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {

    if args.length() != 0 {
        let message = v8::String::new(scope, "Too many arguments".as_ref()).unwrap();
        let exception = v8::Exception::error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    let try_catch = &mut v8::TryCatch::new(scope);

    let output: fn(String) = |_| {};

    let buffer;
    unsafe {
        let state = try_catch.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
        let state = RefCell::borrow(&state);

        let offset = state.modules.get("Command").unwrap().0;
        let memory = state.shared_memory.borrow().unwrap().borrow();
        let size = memory.as_slice()[offset] as usize;
        buffer = &memory.as_slice()[offset + 1..size];
    }

    let source = v8::String::new_from_utf8(try_catch,
                                       buffer,
                                       v8::NewStringType::Normal).unwrap();

    match v8::Script::compile(try_catch, source, Option::None) {
        Some(script) => {
            let result = match script.run(try_catch) {
                Some(result) => result.to_rust_string_lossy(try_catch),
                None => {
                    let exception = try_catch.exception().unwrap();
                    PrettyJsError::create(JsError::from_v8_exception(try_catch, exception)).to_string()
                }
            };
            output(result);
        }
        None => {
            let exception = try_catch.exception().unwrap();
            output(PrettyJsError::create(JsError::from_v8_exception(try_catch, exception)).to_string());
        }
    };
}
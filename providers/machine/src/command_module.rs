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
    let context = v8::Context::new(scope);
    let context_scope = &mut v8::ContextScope::new(scope, context);

    if args.length() != 0 {
        let message = v8::String::new(context_scope, "Too many arguments".as_ref()).unwrap();
        let exception = v8::Exception::error(context_scope, message);
        context_scope.throw_exception(exception);
    }

    let try_catch = &mut v8::TryCatch::new(scope);

    let state = try_catch.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
    let state = RefCell::borrow(&state);

    let output = &state.output;

    let buffer;
    unsafe {
        let offset = state.modules.get("Command").unwrap().0;
        let size = state.shared_memory.unwrap().as_slice()[offset] as usize;
        buffer = &state.shared_memory.unwrap().as_slice()[offset + 1..size];
    }

    let source = v8::String::new_from_utf8(try_catch,
                                           buffer,
                                           v8::NewStringType::Normal).unwrap();

    match v8::Script::compile(try_catch, source, Option::None) {
        Some(script) => {
            output(match script.run(try_catch) {
                Some(result) => result.to_rust_string_lossy(try_catch),
                None => {
                    let exception = try_catch.exception().unwrap();
                    PrettyJsError::create(JsError::from_v8_exception(try_catch, exception)).to_string()
                }
            });
        }
        None => {
            let exception = try_catch.exception().unwrap();
            output(PrettyJsError::create(JsError::from_v8_exception(try_catch, exception)).to_string());
        }
    };
}
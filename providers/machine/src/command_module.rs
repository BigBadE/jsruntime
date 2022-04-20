use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr;
use std::rc::Rc;
use runner::imports::Provider;
use runner::state::JSRunnerState;

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
               args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    if args.length() != 0 {
        let message = v8::String::new(scope, "Too many arguments".as_ref()).unwrap();
        let exception = v8::Exception::error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    let try_catch = &mut v8::TryCatch::new(scope);

    let mut buffer = Vec::new();
    unsafe {
        let state = try_catch.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
        let state = RefCell::borrow(&state);
        let state = state.borrow();

        let offset = state.get_offset("Command");
        let memory = &state.shared_memory;
        let size = memory.as_slice()[offset] as usize;

        buffer.resize(size, 0);
        buffer.copy_from_slice(&memory.as_slice()[offset + 1.. offset + 1 + size]);
        ptr::copy_nonoverlapping([0; 128].as_mut_ptr(), memory.as_ptr(), 128);
    }

    let source = v8::String::new_from_utf8(try_catch,
                                       buffer.as_slice(),
                                       v8::NewStringType::Normal).unwrap();

    match v8::Script::compile(try_catch, source, Option::None) {
        Some(script) => {
            match script.run(try_catch) {
                Some(result) => rv.set(result),
                None => {
                    let exception = try_catch.exception().unwrap();
                    try_catch.throw_exception(exception);
                }
            };
        }
        None => {
            let exception = try_catch.exception().unwrap();
            try_catch.throw_exception(exception);
        }
    };
}
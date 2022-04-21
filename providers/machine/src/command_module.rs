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

    let command;
    unsafe {
        let state = try_catch.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
        let mut state = RefCell::borrow_mut(&state);

        let offset = state.get_offset("Command");
        let memory = &mut state.shared_memory;
        let slice = memory.as_slice_mut();
        if slice[offset + 129] & 0x2 == 0 {
            return;
        }
        slice[offset + 129] ^= 0x2;

        let size = slice[offset] as usize;

        let mut buffer = Vec::new();
        buffer.resize(size, 0);
        buffer.copy_from_slice(&slice[offset + 1.. offset + 1 + size]);
        ptr::copy_nonoverlapping([0; 128].as_mut_ptr(), memory.as_ptr(), 128);

        command = String::from_utf8(buffer).unwrap();
        state.output.log(&command);
    }

    let source = v8::String::new_from_utf8(try_catch,
                                       command.as_bytes(),
                                       v8::NewStringType::Normal).unwrap();

    match v8::Script::compile(try_catch, source, Option::None) {
        Some(script) => {
            match script.run(try_catch) {
                Some(result) => rv.set(result),
                None => {
                    println!("Error!");
                    let exception = try_catch.exception().unwrap();
                    try_catch.throw_exception(exception);
                }
            };
        }
        None => {
            println!("Error!");
            let exception = try_catch.exception().unwrap();
            try_catch.throw_exception(exception);
        }
    };
}
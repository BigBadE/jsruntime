use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{ptr, thread};
use std::time::Duration;
use runner::imports::Provider;
use runner::state::JSRunnerState;

pub fn basic_globals() -> Provider {
    Provider {
        module: Option::None,
        functions: Option::Some(HashMap::from([
            ("print", v8::MapFnTo::map_fn_to(print)),
            ("sync", v8::MapFnTo::map_fn_to(sync))
        ])),
        objects: Option::None,
    }
}

fn sync<'s>(scope: &mut v8::HandleScope<'s>,
            _args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    unsafe {
        let state = scope.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
        let mut state = RefCell::borrow_mut(&state);

        let offset = state.get_offset("Sync");
        let cmd_offset = state.get_offset("Command");
        let pointer = state.output.buffer.as_ptr();

        let updated = state.output.updated;
        state.output.updated = false;

        let memory = &mut state.shared_memory;

        //Write output to shmem
        ptr::copy_nonoverlapping(
            pointer, memory.as_ptr().offset((cmd_offset + 130) as isize),
            runner::logger::SIZE);

        if updated && memory.as_slice()[cmd_offset + 129] & 0x1 == 0 {
            memory.as_slice_mut()[cmd_offset + 129] ^= 0x1;
        }

        while memory.as_slice()[offset] & 0x1 == 0 {
            //Loop until it sync's
            thread::sleep(Duration::new(0, 1));
        }

        //TODO this line is messing it up
        memory.as_slice_mut()[offset] ^= 0x1;
    }
}

fn print<'s>(scope: &mut v8::HandleScope<'s>,
             args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    if args.length() != 1 {
        let message = v8::String::new(scope, "Incorrect arguments".as_ref()).unwrap();
        let exception = v8::Exception::error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    let message = args.get(0).to_rust_string_lossy(scope);
    let state = scope.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
    let mut state = RefCell::borrow_mut(&state);

    state.output.log(&(message + "\n"));
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use runner::imports::Provider;
use runner::state::JSRunnerState;

pub fn basic_globals() -> Provider {
    Provider {
        module: Option::None,
        functions: Option::Some(HashMap::from([("print", v8::MapFnTo::map_fn_to(run_cmd))])),
        objects: Option::None,
    }
}

fn run_cmd<'s>(scope: &mut v8::HandleScope<'s>,
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

    state.output.log(message + "\n");
}
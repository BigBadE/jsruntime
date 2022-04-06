use std::collections::HashMap;
use v8::MapFnTo;
use runner::imports::Provider;

const TARGETS: [&str; 1] = ["file"];

pub fn global_provider() -> Provider {
    Provider {
        name: "core",
        functions: HashMap::from([("$", run_cmd.map_fn_to())]),
        objects: HashMap::new()
    }
}

fn run_cmd<'s>(scope: &mut v8::HandleScope<'s>,
             args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    if args.length() == 0 {
        return;
    }

    let command = args.get(0).to_rust_string_lossy(scope);

    let mut command_args = vec![];

    for i in 1..args.length() {
        command_args.push(args.get(i));
    }

    let context = v8::Context::new(scope);
    let global = context.global(scope);
    let context_scope = &mut v8::ContextScope::new(scope, context);

    let name: v8::Local<v8::Value> = v8::String::new(context_scope, command.as_str()).unwrap().into();

    match global.get(context_scope, name) {
        Some(found) => {
            if found.is_function() {
                call_function(context_scope,
                              v8::Local::<v8::Function>::try_from(found).unwrap(),
                              &mut rv, args.this().into(), &command_args);
                return;
            }
        }
        None => {}
    }

    for target in TARGETS {
        let key = v8::String::new(context_scope, target).unwrap().into();
        let found = global.get(context_scope, key).unwrap();
        if !found.is_object() {
            continue;
        }
        match v8::Local::<v8::Object>::try_from(found).unwrap().get(context_scope, name) {
            Some(found) => {
                if found.is_function() {
                    call_function(context_scope,
                                  v8::Local::<v8::Function>::try_from(found).unwrap(),
                                  &mut rv, args.this().into(), &command_args);
                }
                return;
            }
            None => {

            }
        }
    }

    let message = v8::String::new(context_scope, "No method found".as_ref()).unwrap();
    let exception = v8::Exception::type_error(context_scope, message);
    context_scope.throw_exception(exception);
}

fn call_function(scope: &mut v8::HandleScope, function: v8::Local<v8::Function>,
                 rv: &mut v8::ReturnValue, receiver: v8::Local<v8::Value>, args: &[v8::Local<v8::Value>]) {
    match function.call(scope, receiver, args) {
        Some(value) => {
            rv.set(value);
        }
        None => {}
    }
}
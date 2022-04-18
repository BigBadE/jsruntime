use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use runner::imports::Provider;
use runner::state::JSRunnerState;

const TARGETS: [&str; 1] = ["file"];

pub fn global_provider() -> Provider {
    Provider {
        module: Option::None,
        functions: Option::Some(HashMap::from([("$", v8::MapFnTo::map_fn_to(run_cmd))])),
        objects: Option::None,
    }
}

fn run_cmd<'s>(scope: &mut v8::HandleScope<'s>,
               args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    if args.length() != 1 {
        return;
    }

    let command = args.get(0).to_rust_string_lossy(scope);

    let mut command_args = Vec::new();

    let mut current = command.as_str();

    while current.len() != 0 {
        let index = consume_arg(current);
        command_args.push(v8::String::new(scope, &current[0..index]).unwrap().into());
        current = &current[index+1..];
    }

    {
        let context = scope.get_current_context();
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
                None => {}
            }
        }
    }

    let state = scope.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
    let mut state = RefCell::borrow_mut(&state);
    state.output.log(format!("Unknown command {}", command));
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

fn consume_arg(command: &str) -> usize {
    let mut quoted = false;
    let mut escaped = false;
    let mut i = 0;
    for char in command.chars() {
        if escaped {
            escaped = false;
        } else if char == '\\' {
            escaped = true;
        } else if char == '"' {
            quoted = !quoted;
        } else if char == ' ' && !quoted {
            break;
        }
        i += 1;
    }
    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_consumer() {
        let test_case = "\"one two\"three\"four five\" six";
        let index = consume_arg(test_case);
        assert_eq!(&test_case[0..index], "\"one two\"three\"four five\"");
        assert_eq!(&test_case[index+1..], "six");
    }
}
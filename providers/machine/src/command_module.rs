use std::collections::HashMap;
use runner::imports::Provider;
use runner::runner::JSRunner;

pub fn command_provider() -> Provider {
    Provider {
        module: Option::Some("Command"),
        functions: Option::None,
        objects: Option::Some(HashMap::from([("system", HashMap::from(
            [("run_commands", v8::MapFnTo::map_fn_to(run_cmd))]
        ))]))
    }
}

fn run_cmd<'s>(scope: &mut v8::HandleScope<'s>,
               args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    let context = v8::Context::new(scope);
    let global = context.global(scope);
    let context_scope = &mut v8::ContextScope::new(scope, context);

    if args.length() != 0 {
        let message = v8::String::new(context_scope, "Too many arguments".as_ref()).unwrap();
        let exception = v8::Exception::error(context_scope, message);
        context_scope.throw_exception(exception);
    }

    JSRunner::run_with_scope(v8::HandleScope::with_context(&mut self.isolate, context));
}
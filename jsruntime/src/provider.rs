use std::collections::HashMap;

pub trait Provider {
    fn global_functions<'s>(&self) -> HashMap<&'s [u8],
        &'s dyn Fn(&v8::HandleScope<'s>, v8::FunctionCallbackArguments, v8::ReturnValue)>;
}

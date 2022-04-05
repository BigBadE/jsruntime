use std::collections::HashMap;

pub struct Provider<'s> {
    pub name: String,
    pub globals: HashMap<str, Vec<&'s dyn Fn(&v8::HandleScope<'s>,
        v8::FunctionCallbackArguments, v8::ReturnValue)>>
}

pub fn providers() -> Vec<Provider> {
    //All structs providing imports
    vec!()
}
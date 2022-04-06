use std::collections::HashMap;

pub struct Provider<'s> {
    pub name: String,
    pub globals: HashMap<String, Vec<&'s dyn Fn(&mut v8::HandleScope<'s>,
        v8::FunctionCallbackArguments, v8::ReturnValue)>>
}

pub fn providers<'s>() -> Vec<Provider<'s>> {
    //All structs providing imports
    vec!()
}
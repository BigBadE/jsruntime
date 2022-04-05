use std::collections::HashMap;

use crate::provider::Provider;

pub fn register_imports<'s>(global_functions: &mut HashMap<&'s [u8],
    &dyn Fn(&mut v8::HandleScope<'_>, v8::FunctionCallbackArguments, v8::ReturnValue)>) {

    //All structs providing imports
    let providers: Vec<Box<dyn Provider>> = vec!();

    for provider in providers {
        for (key, value) in provider.global_functions() {
            global_functions.insert(key, value);
        }
    }
}
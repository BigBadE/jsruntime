use std::collections::HashMap;

use crate::provider::Provider;

//All structs providing imports
static PROVIDERS: Vec<Box<dyn Provider>> = vec!();

pub fn register_imports<'s>(mut global_functions: &HashMap<&'s [u8],
    &'s dyn Fn(&v8::HandleScope<'s>, v8::FunctionCallbackArguments, v8::ReturnValue)>) {

    for provider in PROVIDERS {
        for (key, value) in provider.global_functions() {
            global_functions.insert(key, value);
        }
    }
}
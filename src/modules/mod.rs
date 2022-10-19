use std::collections::HashMap;
use crate::modules::core_modules::command_module;

mod core_modules;

pub struct Module {
    pub name: String,
    pub objects: Vec<&'static str>,
    pub functions: HashMap<&'static str, v8::FunctionCallback>
}

pub fn modules() -> Vec<Module> {
    vec!(command_module())
}

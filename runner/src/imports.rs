use std::collections::HashMap;

pub struct Provider {
    pub name: String,
    pub globals: HashMap<String, Vec<v8::FunctionCallback>>
}

pub fn providers() -> Vec<Provider> {
    //All structs providing imports
    vec!()
}
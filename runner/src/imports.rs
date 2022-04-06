use std::collections::HashMap;

pub struct Provider {
    pub name: &'static str,
    pub functions: HashMap<&'static str, v8::FunctionCallback>,
    pub objects: HashMap<&'static str, Vec<v8::FunctionCallback>>
}
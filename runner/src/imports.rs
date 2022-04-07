use std::collections::HashMap;

pub struct Provider {
    pub module: Option<&'static str>,
    pub functions: Option<HashMap<&'static str, v8::FunctionCallback>>,
    pub objects: Option<HashMap<&'static str, HashMap<&'static str, v8::FunctionCallback>>>,
}
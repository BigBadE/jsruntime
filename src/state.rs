use std::collections::HashMap;

//Get external functions with
//unsafe { std::mem::transmute(state.external_functions[&name.to_string()]) }
pub struct JSRunnerState {
    pub global_context: v8::Global<v8::Context>,
    pub output: *const (),
    pub external_functions: HashMap<String, *const ()>,
    pub id: i32
}
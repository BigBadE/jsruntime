use std::collections::HashMap;
use std::slice;

//Get external functions with
//unsafe { std::mem::transmute(state.external_functions[&name.to_string()]) }
pub struct JSRunnerState {
    pub global_context: v8::Global<v8::Context>,
    pub output: *const (),
    pub external_functions: HashMap<String, *const ()>,
    pub id: i32
}

#[repr(C, packed(1))]
pub struct ArrayStruct<T> where T : Sized {
    pub pointer: *mut T,
    pub length: i32
}

impl<T> ArrayStruct<T> {
    pub unsafe fn Deserialize(&self) -> Vec<T> {
        return Vec::from_raw_parts(self.pointer, self.length as usize, self.length as usize)
    }
}
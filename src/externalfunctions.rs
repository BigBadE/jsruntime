use std::collections::HashMap;
use std::fmt::Error;
use std::ops::Add;
use std::ptr;

#[repr(C, packed(1))]
pub struct ExternalFunctions {
    pub function_keys: *const u32,
    pub function_values: *const u32,
    pub functions_length: i32,
    pub objects: *const u32,
    pub object_lengths: i32,
    pub path: *const u32,
    pub path_length: i32,
}

impl ExternalFunctions {
    pub fn get_path(self) -> Result<String, Error> {
        let parts: &[u16] = unsafe { std::slice::from_raw_parts(self.path as *const _, self.path_length as usize) };

        return match String::from_utf16(parts) {
            Ok(result) => Ok(result),
            Err(_error) => Err(Error::default())
        };
    }

    pub fn get_functions(self) -> HashMap<String, *const u32> {
        let map = HashMap::with_capacity(self.functions_length as usize);
        let mut current = self.function_keys;
        for i in 0..self.functions_length {
            unsafe {
                let size = ptr::read(current);
                current.add(8);
                let parts: &[u16] = std::slice::from_raw_parts(current as *const _, self.path_length as usize);
                current.add((size * 4) as usize);
            }
        }
        return map;
    }
}
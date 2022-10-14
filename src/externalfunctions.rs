use std::collections::HashMap;
use std::{mem, ptr};
use anyhow::Error;

#[derive(Clone, Copy)]
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
            Err(_error) => Err(Error::msg(_error.to_string()))
        };
    }

    pub fn get_objects(self) -> Result<Vec<String>, Error> {
        Ok(self.load_string(self.objects, self.object_lengths as usize)?)
    }

    pub fn load_string(self, pointer: *const u32, length: usize) -> Result<Vec<String>, Error> {
        let mut result = Vec::with_capacity(length);
        let mut current = pointer;

        for _ in 0..length {
            unsafe {
                let size = ptr::read(current);
                current = current.add(8);
                let parts: &[u16] = std::slice::from_raw_parts(current as *const _, self.path_length as usize);
                current = current.add((size * 4) as usize);
                result.push(String::from_utf16(parts)?);
            }
        }
        return Ok(result);
    }

    pub fn get_functions(self) -> Result<HashMap<String, *const u32>, Error> {
        let mut map = HashMap::with_capacity(self.functions_length as usize);
        let mut current = self.function_keys;

        let mut keys = self.load_string(self.function_keys, self.functions_length as usize)?;

        for i in 0..self.functions_length {
            unsafe {
                let pointer = ptr::read(current);
                map.insert(mem::replace(&mut keys[i as usize], String::new()), pointer as *const u32);
                current = current.add(8);
            }
        }

        return Ok(map);
    }
}
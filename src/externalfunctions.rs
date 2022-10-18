use std::collections::HashMap;
use std::slice;
use anyhow::Error;

#[repr(C, packed(1))]
pub struct ExternalFunctions {
    pub function: HashMap<String, *const u32>,
    pub objects: Vec<String>,
    pub path: String
}

impl ExternalFunctions {
    pub unsafe fn new(mut function_keys: *const u16, function_values: *const *const (),
               function_sizes: *const u16, functions_length: i32,
               mut objects: *const u16, object_sizes: *const u16, object_length: i32,
               path: *const u16, path_length: i32) -> Result<Self, Error> {
        let mut created = ExternalFunctions {
            function: HashMap::with_capacity(functions_length as usize),
            objects: Vec::with_capacity(object_length as usize),
            path: String::new()
        };

        created.path = get_string(slice::from_raw_parts(path, path_length as usize))?;

        let sizes = slice::from_raw_parts(object_sizes, object_length as usize);
        for i in 0..object_length as usize {
            let size = sizes[i] as usize;
            created.objects.push(get_string(slice::from_raw_parts(objects, size))?);
            objects = objects.add((size * 4) as usize);
        }

        let sizes = slice::from_raw_parts(function_sizes, functions_length as usize);
        let values = slice::from_raw_parts(function_values, functions_length as usize);
        for i in 0..functions_length as usize {
            let size = sizes[i] as usize;
            created.function.insert(get_string(slice::from_raw_parts(function_keys, size))?, values[i]);
            function_keys = function_keys.add((size * 4) as usize);
        };

        return Ok(created);
    }
}

fn get_string(input: &[u16]) -> Result<String, Error> {
    return match String::from_utf16(input) {
        Ok(str) => Ok(str),
        Err(error) => Err(Error::msg(error.to_string()))
    };
}
use std::collections::HashMap;
use std::slice;
use anyhow::Error;

pub struct ExternalFunctions {
    pub modules: Vec<String>,
    pub function: HashMap<String, *const ()>,
    pub path: String,
    pub machine_id: i32
}

impl ExternalFunctions {
    pub unsafe fn new(mut function_keys: *const u16, function_values: *const *const (),
               function_sizes: *const u16, functions_length: i32,
               mut modules: *const u16, module_sizes: *const u16, module_length: i32,
               path: *const u16, path_length: i32, machine_id: i32) -> Result<Self, Error> {
        let mut created = ExternalFunctions {
            function: HashMap::with_capacity(functions_length as usize),
            modules: Vec::with_capacity(module_length as usize),
            path: String::new(),
            machine_id
        };

        created.path = get_string(path, path_length as u16)?;

        let sizes = slice::from_raw_parts(module_sizes, module_length as usize);
        for i in 0..module_length as usize {
            created.modules.push(get_string(modules, sizes[i])?);
            modules = modules.add((sizes[i] * 4) as usize);
        }

        let sizes = slice::from_raw_parts(function_sizes, functions_length as usize);
        let values = slice::from_raw_parts(function_values, functions_length as usize);
        for i in 0..functions_length as usize {
            created.function.insert(get_string(function_keys, sizes[i])?, values[i]);
            function_keys = function_keys.add((sizes[i] * 4) as usize);
        };

        return Ok(created);
    }
}

pub fn get_string(pointer: *const u16, length: u16) -> Result<String, Error> {
    return match String::from_utf16(unsafe { slice::from_raw_parts(pointer, length as usize) }) {
        Ok(str) => Ok(str),
        Err(error) => Err(Error::msg(error.to_string()))
    };
}
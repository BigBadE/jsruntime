
use std::fmt::Error;

pub struct ExternalFunctions {
    pub function_keys: *const u32,
    pub function_values: *const u32,
    pub functions_length: usize,
    pub objects: *const u32,
    pub object_lengths: usize,
    pub path: *mut u8,
    pub path_length: usize,
}

impl ExternalFunctions {
    pub fn get_path(self) -> Result<String, Error> {
        return
            match unsafe { String::from_utf8(Vec::from_raw_parts(self.path, self.path_length, self.path_length)) } {
                Ok(result) => Ok(result),
                Err(_error) => Err(Error::default())
            };
    }
}
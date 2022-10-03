use std::fmt::Error;

pub struct ExternalFunctions {
    pub function_keys: *const u32,
    pub function_values: *const u32,
    pub functions_length: usize,
    pub objects: *const u32,
    pub object_lengths: usize,
    pub path: *const u32,
    pub path_length: usize,
}

impl ExternalFunctions {
    pub fn get_path(self) -> Result<String, Error> {
        let parts: &[u16] = unsafe { std::slice::from_raw_parts(self.path as *const _, self.path_length) };

        return match String::from_utf16(parts) {
            Ok(result) => Ok(result),
            Err(_error) => Err(Error::default())
        };
    }
}
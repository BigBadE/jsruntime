use std::fmt::Error;

#[repr(C)]
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
        let parts: &[u16] = unsafe { std::slice::from_raw_parts(self.path as *const _, self.path_length) };

        return match String::from_utf16(parts) {
            Ok(result) => Ok(result),
            Err(_error) => Err(Error::default())
        };
    }
}
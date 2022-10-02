pub struct ExternalFunctions {
    pub function_keys: *const u32,
    pub function_values: *const u32,
    pub functions_length: usize,
    pub objects: *const u32,
    pub object_lengths: usize,
    pub path: *const u32,
    pub path_length: usize,
}
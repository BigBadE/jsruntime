use std::collections::HashMap;
use std::{mem, ptr};
use std::ffi::CString;
use std::os::raw::c_char;
use anyhow::Error;
use crate::log;

#[derive(Clone, Copy)]
#[repr(C, packed(1))]
pub struct ExternalFunctions {
    pub function_keys: *mut [CString],
    pub function_values: *const [u16],
    pub functions_length: i32,
    pub objects: *mut [CString],
    pub object_lengths: i32,
    pub path: *mut c_char
}

impl ExternalFunctions {
}
#![no_main]

mod jsrunner;
mod state;
mod externalfunctions;

use std::fs;
use std::path::Path;
use anyhow::Error;
use crate::externalfunctions::ExternalFunctions;
use crate::jsrunner::JSRunner;

#[no_mangle]
pub extern "C" fn serenity_run(mut function_keys: *const u16, function_values: *const *const (),
                               function_sizes: *const u16, functions_length: i32,
                               mut objects: *const u16, object_sizes: *const u16, object_length: i32,
                               path: *const u16, path_length: i32, logger: *const ()) {
    let externals = unsafe {
        ExternalFunctions::new(function_keys, function_values, function_sizes, functions_length,
                               objects, object_sizes, object_length, path, path_length)?
    };

    match run_internal(externals, logger) {
        Err(error) =>
            log(logger, format!("{}", error).as_str()),
        _ => {}
    }
}


fn run_internal(externals: ExternalFunctions, logger: *const ()) -> Result<bool, Error> {
    let params = v8::Isolate::create_params()
        .array_buffer_allocator(v8::new_default_allocator())
        .allow_atomics_wait(false)
        .heap_limits(0, 3 * 1024 * 1024);

    let mut runner = JSRunner::new(None, params, externals.clone(), logger)?;

    //let path = externals.get_path()?;


    return /*match fs::read_to_string(Path::new(&path)) {
        Ok(source) => {*/
            Ok(runner.run("1+1=2".as_bytes())?.is_array());/*
            Ok(true)
        }
        Err(error) => Err(Error::msg(format!("Error: {}", error)))
    };*/
}

fn log(logger: *const (), logging: &str) {
    let logger: fn(*const (), i32) = unsafe { std::mem::transmute(logger) };
    logger(logging as *const str as *const (), logging.len() as i32)
}
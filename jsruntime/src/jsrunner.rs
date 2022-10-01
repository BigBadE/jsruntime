use std::{io};
use std::fs::File;
use std::io::Write;
use runner::runner::JSRunner;

#[no_mangle]
pub extern "C" fn serenity_init(functions_ptr: *const u32, identifiers_ptr: *const u32, functions_len: usize) {
    let identifiers = unsafe { std::slice::from_raw_parts(identifiers_ptr, functions_len) };
    let functions = unsafe { std::slice::from_raw_parts(functions_ptr, functions_len) };

}

#[no_mangle]
pub extern "C" fn serenity_run(path_ptr: *const u32, path_len: usize, logger: *const u32) {
    let path = unsafe { (std::slice::from_raw_parts(path_ptr, path_len)) };

    /*
    let function: fn() = unsafe { std::mem::transmute(logger) };
    (function)();
    */

    let params = v8::Isolate::create_params()
        .array_buffer_allocator(v8::new_default_allocator())
        .allow_atomics_wait(false)
        .heap_limits(0, 3 * 1024 * 1024);

    let mut runner = JSRunner::new(Option::None, params, logger.clone());

    return match fs::read_to_string(Path::new(path)) {
        Ok(source) => {
            match runner.run(source.as_bytes()) {
                Err(error) => Option::Some(Error::from(error)),
                _ => Option::None
            }
        }
        Err(error) => Option::Some(Error::msg(format!("{} for {}", error, path)))
    };
}

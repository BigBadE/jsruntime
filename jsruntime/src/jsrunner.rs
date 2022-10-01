use std::{io};
use std::fs::File;
use std::io::Write;

#[no_mangle]
pub extern "C" fn serenity_run(logger: *const i8) {
    let function: fn() = unsafe { std::mem::transmute(logger) };
    (function)();
    /*
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
    };*/
}

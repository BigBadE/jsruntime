use std::fs;
use std::path::Path;
use anyhow::Error;
use runner::runner::JSRunner;

#[no_mangle]
pub fn serenity_run(path: &String, logger: &i8) -> &'static str {
    return "Run success!";
    /*
    let function = logger.clone() as *const ();
    let function: fn(&str) -> i32 = unsafe { std::mem::transmute(function) };
    (function)("Testing");

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

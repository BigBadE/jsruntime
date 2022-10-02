use std::fmt::Error;
use crate::externalfunctions::ExternalFunctions;

mod externalfunctions;

#[no_mangle]
pub extern "C" fn serenity_run(external_functions: ExternalFunctions, logger: *const u32) {
    let function: fn(*const u32, usize) = unsafe { std::mem::transmute(logger) };
    let log = |input: &str| function(input as *const str as *const u32, input.len());

    match serenity_run_internal(external_functions, &log) {
        Ok(output) => log("Success!"),
        Err(error) => log("Error")
    }
}

fn serenity_run_internal(external_functions: ExternalFunctions, logger: &dyn Fn(&str)) -> Result<bool, Error> {

    let printing = external_functions.get_path()?;
    logger(printing.as_str());

    Ok(true)
    /*let params = v8::Isolate::create_params()
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

use std::fs;
use std::path::Path;
use anyhow::Error;
use runner::runner::JSRunner;
use crate::externalfunctions::ExternalFunctions;

mod externalfunctions;

#[no_mangle]
pub extern "C" fn serenity_run(external_functions: ExternalFunctions, logger: *const u32) {
    print(logger, "Starting Serenity");
    match serenity_run_internal(external_functions, logger) {
        Err(error) => {
            let maybe_error = error.chain().last();
            match maybe_error {
                Some(error) => print(logger, error.to_string().as_str()),
                None => print(logger, "Error with no error given!")
            }
        }
        _ => {}
    }
}

fn print(logger: *const u32, printing: &str) {
    let function: fn(*const u32, usize) = unsafe { std::mem::transmute(logger) };
    (function)(printing as *const str as *const u32, printing.len());
}

fn serenity_run_internal(external_functions: ExternalFunctions, logger: *const u32) -> Result<bool, Error> {
    let printing = external_functions.get_path()?;

    /*let params = v8::Isolate::create_params()
        .array_buffer_allocator(v8::new_default_allocator())
        .allow_atomics_wait(false)
        .heap_limits(0, 3 * 1024 * 1024);

    let mut runner = JSRunner::new(None, params, logger);

    return match fs::read_to_string(Path::new(&printing)) {
        Ok(source) => {
            match runner.run(source.as_bytes()) {
                Err(error) => Err(Error::from(error)),
                _ => Ok(true)
            }
        }
        Err(error) => Err(Error::msg(format!("{} for {}", error, printing)))
    };*/
    Ok(true)
}

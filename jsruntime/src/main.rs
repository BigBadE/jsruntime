use std::collections::HashMap;
use std::{fs, io};
use std::io::Read;
use std::path::Path;
use std::thread::Thread;
use runner::runner::JSRunner;

pub mod imports;
pub mod provider;


fn main() {
    let mut processes: HashMap<&str, Thread> = HashMap::new();

    let mut args = Vec::new();
    loop {
        io::stdin().read_to_end(&mut args).unwrap();
        let output = String::from_utf8_lossy(&args).into_owned();
        let split: Vec<&str> = output.split('\u{0000}').collect();
        if split.len() == 1 {
            match processes.get(split[0]) {
                Some(value) => {

                }
                None => {
                    run(&String::from(split[0]), Option::None,
                        vec![]);
                }
            }
        } else {
            run(&String::from(split[0]), Option::Some(&String::from(split[1])),
                split[2].split(",").collect());
        }
    }
}

fn run(path: &String, memoryMap: Option<&String>, modules: Vec<&str>) {
    let params = v8::Isolate::create_params()
        .array_buffer_allocator(v8::new_default_allocator())
        .allow_atomics_wait(false)
        .heap_limits(0, 3 * 1024 * 1024);

    let globals: HashMap<&[u8],
        &dyn Fn(&mut v8::HandleScope<'_>, v8::FunctionCallbackArguments<'_>, v8::ReturnValue<'_>)> =
        HashMap::new();

    let mut runner = JSRunner::new(
        Option::None, params, globals);

    let _result = runner.run(fs::read_to_string(Path::new(
        path)).unwrap().as_bytes());
}

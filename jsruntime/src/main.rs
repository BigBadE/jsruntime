use std::collections::HashMap;
use std::{fs, io, thread};
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use shared_memory::ShmemConf;
use machine::global_provider::global_provider;
use runner::runner::JSRunner;
use runner::imports::Provider;

fn main() {
    let mut args = Vec::new();
    let processes: Arc<RwLock<HashMap<String, JoinHandle<()>>>> =
        Arc::new(RwLock::new(HashMap::new()));

    loop {
        io::stdin().read_to_end(&mut args).unwrap();
        let output = String::from_utf8_lossy(&args).into_owned();
        let mut map = processes.write().unwrap();
        if !output.contains('\u{0000}') {
            if !map.contains_key(output.as_str()) {
                map.insert(
                    output.clone(),
                    thread::spawn(move || {
                        run(&output,
                            Option::None, vec![])
                    }));
                continue;
            }
            map.remove(output.as_str());
        } else {
            map.insert(
                String::from(output.split('\u{0000}').next().unwrap()),
                thread::spawn(move || {
                    let split: Vec<&str> = output.split('\u{0000}').collect();
                    run(&String::from(split[0]), Option::Some(String::from(split[1])),
                        split[2].split(",").collect())
                }));
        }
    }
}

pub fn providers() -> Vec<Provider> {
    //All structs providing imports
    vec!(global_provider())
}

fn run(path: &String, memory_map: Option<String>, _modules: Vec<&str>) {
    let params = v8::Isolate::create_params()
        .array_buffer_allocator(v8::new_default_allocator())
        .allow_atomics_wait(false)
        .heap_limits(0, 3 * 1024 * 1024);

    let providers = providers();

    let memory;

    match memory_map {
        Some(path) => memory = Option::Some(ShmemConf::new().os_id(&path).create().unwrap()),
        None => memory = Option::None
    }
    let mut runner = JSRunner::new(
        Option::None, params, providers, memory);

    let _result = runner.run(fs::read_to_string(Path::new(
        path)).unwrap().as_bytes());
}

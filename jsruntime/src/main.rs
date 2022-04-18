use std::collections::HashMap;
use std::{fs, io, thread};
use std::path::Path;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::thread::JoinHandle;
use anyhow::Error;
use shared_memory::ShmemConf;
use machine::basic_globals::basic_globals;
use machine::command_module::command_provider;
use machine::global_provider::global_provider;
use runner::runner::JSRunner;
use runner::imports::Provider;

pub fn providers() -> Vec<Provider> {
    //All structs providing imports
    vec!(global_provider(), command_provider(), basic_globals())
}

fn main() {
    let processes: Arc<RwLock<HashMap<String, JoinHandle<()>>>> =
        Arc::new(RwLock::new(HashMap::new()));

    loop {
        let mut output = String::new();
        io::stdin().read_line(&mut output).unwrap();
        start_process(processes.write().unwrap(), output);
    }
}

fn start_process(mut map: RwLockWriteGuard<HashMap<String, JoinHandle<()>>>, output: String) {
    if !output.contains('\u{0000}') {
        if !map.contains_key(output.as_str()) {
            map.insert(
                output.clone(),
                thread::Builder::new().name(output.clone()).spawn(move || {
                    let result = run(&output,
                            Option::None, vec![]);
                    if result.is_some() {
                        println!("{}", result.unwrap());
                    }
                }).unwrap());
            return;
        }
        map.remove(output.as_str());
    } else {
        let name = String::from(output.split('\u{0000}').next().unwrap());
        map.insert(
            name.clone(),
            thread::Builder::new().name(name.clone()).spawn(move || {
                let split: Vec<&str> = output.split('\u{0000}').collect();
                let result = run(&name, Option::Some(String::from(split[1])),
                    split[2].split(",").collect());
                if result.is_some() {
                    println!("{}", result.unwrap());
                }
            }).unwrap());
    }
}

fn run(path: &String, memory_map: Option<String>, modules: Vec<&str>) -> Option<Error> {
    let params = v8::Isolate::create_params()
        .array_buffer_allocator(v8::new_default_allocator())
        .allow_atomics_wait(false)
        .heap_limits(0, 3 * 1024 * 1024);

    let mut module_sizes = HashMap::new();

    let mut allowed_modules = vec!();

    let mut i = 0;
    for module in modules {
        let split;
        match module.find(':') {
            Some(found) => split = found,
            None => continue
        };
        let size;
        match module[split+1..].parse::<usize>() {
            Ok(found) => size = found,
            Err(error) => return Option::Some(
                Error::msg(format!("{} for usize {}", error, &module[split+1..])))
        }

        allowed_modules.push(&module[..split]);

        module_sizes.insert(module[0..split].to_string(),
                            (i, size));
        i += size;
    }

    let mut found_providers = vec!();

    for provider in providers() {
        match provider.module {
            Some(module) => {
                if allowed_modules.contains(&module) {
                    found_providers.push(provider);
                }
            }
            _ => found_providers.push(provider)
        }
    }

    let memory;

    match memory_map {
        Some(memory_path) => {
            match ShmemConf::new().os_id(memory_path).size(i).create() {
                Ok(mem) => memory = Option::Some(mem),
                Err(error) => return Option::Some(Error::new(error))
            }
        },
        None => memory = Option::None
    }

    let mut runner = JSRunner::new(
        Option::None, params, found_providers, memory, module_sizes);

    return match fs::read_to_string(Path::new(path)) {
        Ok(source) => {
            match runner.run(source.as_bytes()) {
                Err(error) => runner.log(error.to_string()),
                _ => {}
            }
            Option::None
        },
        Err(error) => Option::Some(Error::msg(format!("{} for {}", error, path)))
    }
}

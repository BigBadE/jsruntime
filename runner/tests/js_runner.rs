extern crate core;


use std::collections::HashMap;
use std::fmt::{Display};
use std::{fs, io};
use serde::Deserialize;
use v8::ValueDeserializer;
use runner::runner::JSRunner;
use serde_v8::{Deserializer, Error};

#[test]
fn run_js_tests() {
    let paths = fs::read_dir("./tests/js").unwrap();

    let params = v8::Isolate::create_params()
        .array_buffer_allocator(v8::new_default_allocator())
        .allow_atomics_wait(false)
        .heap_limits(0, 3 * 1024 * 1024);

    let mut runner = JSRunner::new(Option::None, params, HashMap::from([
        ("print".as_bytes(), print)
    ]));

    for path in paths {
        let path = path.unwrap().path();
        println!("Running {}", path.to_str().unwrap());
        let result = runner.run(fs::read_to_string(path).unwrap().as_bytes());
        if result.is_err() {
            print!("{}\n", result.err().unwrap());
            assert!(false)
        }
        //assert!(false)
    }
}

fn print<'s>(scope: &mut v8::HandleScope<'s>,
                                           args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    println!("{}", args.get(0).to_rust_string_lossy(scope));
}
use std::collections::HashMap;
use runner::runner::JSRunner;

pub mod imports;
pub mod provider;

fn main() {
    /*let paths = fs::read_dir("./tests/js").unwrap();

    let params = v8::Isolate::create_params()
        .array_buffer_allocator(v8::new_default_allocator())
        .allow_atomics_wait(false)
        .heap_limits(0, 3 * 1024 * 1024);

    let globals = HashMap::new();

    let mut runner = JSRunner::new(Option::None, params, imports.get_HashMap::from([
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
    }*/
}

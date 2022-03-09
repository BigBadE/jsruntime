use std::fs;
use runner::runner::JSRunner;
use util::error::JsError;
use util::fmt_error::PrettyJsError;

#[test]
fn run_js_tests() {
    let paths = fs::read_dir("./tests/js").unwrap();

    let mut runner = JSRunner::new(Option::None, 256, 8000);

    for path in paths {
        let path = path.unwrap().path();
        print!("Running {}\n", path.to_str().unwrap());
        let result = runner.run(fs::read_to_string(path).unwrap().as_bytes());
        if result.is_err() {
            print!("{}\n", result.err().unwrap());
            assert!(false)
        }
    }
    assert!(false)
}
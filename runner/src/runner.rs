use anyhow::Error;
use util::error::JsError;

static INITIALIZED: bool = false;

pub struct JSRunner {
    isolate: v8::OwnedIsolate
}

impl JSRunner {
    pub fn new(platform: Option<v8::SharedRef<v8::Platform>>, heap_min: usize, heap_max: usize) -> Self {
        if !INITIALIZED {
            JSRunner::initialize(platform)
        }

        let mut params = v8::Isolate::create_params()
            .array_buffer_allocator(v8::new_default_allocator())
            .allow_atomics_wait(false)
            .heap_limits(heap_min, heap_max);

        JSRunner {
            isolate: v8::Isolate::new(params)
        }
    }

    pub fn run(&mut self, source: &[u8]) -> Result<v8::Local<v8::Value>, Error>{
        let isolate = &mut self.isolate;
        let mut scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(scope);
        let mut handle_scope = &mut v8::HandleScope::with_context(isolate, context);

        let source = v8::String::new_from_utf8(scope, source,
                                               v8::NewStringType::Normal).unwrap();

        let try_catch = &mut v8::TryCatch::new(handle_scope);

        let script = match v8::Script::compile(try_catch, source, Option::None) {
            Some(script) => script,
            None => return Result::Err(Error::new(
                JsError::from_v8_exception(try_catch, try_catch.exception().unwrap())))
        };

        match script.run(try_catch) {
            Some(result) => Result::Ok(result),
            None => Result::Err(Error::new(
                JsError::from_v8_exception(try_catch, try_catch.exception().unwrap())))
        }
    }

    fn initialize(platform: Option<v8::SharedRef<v8::Platform>>) {
        // Include 10MB ICU data file.
        #[repr(C, align(16))]
        struct IcuData([u8; 10144432]);
        static ICU_DATA: IcuData = IcuData(*include_bytes!("../icudtl.dat"));
        v8::icu::set_common_data_69(&ICU_DATA.0).unwrap();

        match platform {
            None => v8::V8::initialize_platform(
                v8::new_default_platform(0, false).make_shared()),
            Some(platform) =>
                v8::V8::initialize_platform(platform)
        }

        v8::V8::initialize();
    }
}
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use anyhow::Error;
use lazy_static::lazy_static;
use crate::{ExternalFunctions, log};
use crate::state::JSRunnerState;

lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub struct JSRunner {
    isolate: v8::OwnedIsolate
}

impl JSRunner {
    pub fn new(platform: Option<v8::SharedRef<v8::Platform>>, params: v8::CreateParams,
               externals: ExternalFunctions, logger: *const ()) -> Self {
        {
            log(logger, "Initializing");
            let mut init = INITIALIZED.lock().unwrap();
            if *init == false {
                JSRunner::initialize(platform);
                *init = true;
            }
        }

        let mut isolate = v8::Isolate::new(params);

        let global_context;
        {
            let scope = &mut v8::HandleScope::new(&mut isolate);

            let context = v8::Context::new(scope);

            let context_scope = &mut v8::ContextScope::new(scope, context);

            global_context = v8::Global::new(context_scope, context)
        }

        isolate.set_slot(Rc::new(RefCell::new(JSRunnerState {
            global_context,
            output: logger
        })));

        return JSRunner {
            isolate
        };
    }

    /// Runs the given script on the current isolate
    pub fn run(&mut self, source: &[u8]) -> Result<v8::Local<v8::Value>, Error> {
        let handle_scope = &mut self.handle_scope();

        let source = v8::String::new_from_utf8(handle_scope, source,
                                               v8::NewStringType::Normal).unwrap();

        let try_catch = &mut v8::TryCatch::new(handle_scope);

        let script = match v8::Script::compile(try_catch, source, None) {
            Some(script) => script,
            None => {
                let exception = try_catch.exception().unwrap();
                let scope = &mut v8::HandleScope::new(try_catch);
                return Result::Err(Error::msg(v8::Exception::create_message(scope, exception).get(scope).to_rust_string_lossy(scope)));
            }
        };

        match script.run(try_catch) {
            Some(result) => Result::Ok(result),
            None => {
                let exception = try_catch.exception().unwrap();
                let scope = &mut v8::HandleScope::new(try_catch);
                return Result::Err(Error::msg(v8::Exception::create_message(scope, exception).get(scope).to_rust_string_lossy(scope)))
            }
        }
    }

    /// Initializes V8 engine
    fn initialize(platform: Option<v8::SharedRef<v8::Platform>>) {
        // Include 10MB ICU data file.
        #[repr(C, align(16))]
        struct IcuData([u8; 10284336]);
        static ICU_DATA: IcuData = IcuData(*include_bytes!("../icudtl.dat"));
        v8::icu::set_common_data_70(&ICU_DATA.0).unwrap();

        match platform {
            None => v8::V8::initialize_platform(
                v8::new_default_platform(0, false).make_shared()),
            Some(platform) =>
                v8::V8::initialize_platform(platform)
        }

        v8::V8::initialize();
    }

    /// Appears to be bugged, SEGMENTATION FAULTs
    pub fn shutdown() {
        unsafe {
            v8::V8::dispose();
        }
        v8::V8::dispose_platform();
    }

    /// Gets the global context of the current isolate
    fn global_context(&mut self) -> v8::Global<v8::Context> {
        let state = Self::get_state(&mut self.isolate);
        let state = RefCell::borrow(&state);
        state.global_context.clone()
    }

    /// Gets the JSRunnerState of the isolate
    fn get_state(isolate: &v8::Isolate) -> Rc<RefCell<JSRunnerState>> {
        let s = isolate.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
        s.clone()
    }

    /// Returns the handle scope
    pub fn handle_scope(&mut self) -> v8::HandleScope {
        let context = self.global_context();
        v8::HandleScope::with_context(&mut self.isolate, context)
    }

    pub fn log(self, message: &String) {
        let state = JSRunner::get_state(&self.isolate);
        let state = RefCell::borrow_mut(&state);
        let function: fn(*const u32, usize) = unsafe { std::mem::transmute(state.output) };
        (function)(message.as_str() as *const str as *const u32, message.len());
    }
}
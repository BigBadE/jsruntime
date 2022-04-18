use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use anyhow::Error;
use shared_memory::Shmem;
use v8::{CreateParams, Object};
use util::error::JsError;
use util::fmt_error::PrettyJsError;
use crate::imports::Provider;
use crate::logger::Logger;
use crate::state::JSRunnerState;

static INITIALIZED: bool = false;

pub struct JSRunner {
    isolate: v8::OwnedIsolate,
}

impl JSRunner {
    pub fn new(platform: Option<v8::SharedRef<v8::Platform>>, params: CreateParams,
               providers: Vec<Provider>, shared_memory: Option<Shmem>,
               modules: HashMap<String, (usize, usize)>) -> Self {
        if !INITIALIZED {
            JSRunner::initialize(platform)
        }

        let mut isolate = v8::Isolate::new(params);

        let global_context;
        {
            let scope = &mut v8::HandleScope::new(&mut isolate);

            let context = v8::Context::new(scope);

            let global = context.global(scope);

            let context_scope = &mut v8::ContextScope::new(scope, context);

            for provider in providers {
                match provider.objects {
                    Some(objects) => {
                        for (name, functions) in objects {
                            let global_key =
                                v8::String::new(context_scope, name).unwrap().into();

                            let object: v8::Local<Object> = match global.get(context_scope, global_key) {
                                Some(found) => found.try_into().unwrap(),
                                None => Object::new(context_scope)
                            };

                            for (func_name, function) in functions {
                                set_func(context_scope, object, func_name, function);
                            }
                            println!("Added object {}", name);
                            global.set(context_scope, global_key,
                                       object.into());
                        }
                    }
                    _ => {}
                }
                match provider.functions {
                    Some(functions) => {
                        for (name, function) in functions {
                            set_func(context_scope, global, name, function)
                        }
                    }
                    _ => {}
                }
            }
            global_context = v8::Global::new(context_scope, context)
        }

        isolate.set_slot(Rc::new(RefCell::new(JSRunnerState {
            global_context,
            shared_memory,
            modules,
            output: Logger::new()
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

        let script = match v8::Script::compile(try_catch, source, Option::None) {
            Some(script) => script,
            None => {
                let exception = try_catch.exception().unwrap();
                return Result::Err(
                    PrettyJsError::create(JsError::from_v8_exception(try_catch, exception)));
            }
        };

        match script.run(try_catch) {
            Some(result) => Result::Ok(result),
            None => {
                let exception = try_catch.exception().unwrap();
                return Result::Err(PrettyJsError::create(
                    JsError::from_v8_exception(try_catch, exception)));
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

    pub fn log(self, message: String) {
        let state = JSRunner::get_state(&self.isolate);
        let mut state = RefCell::borrow_mut(&state);
        state.output.log(message);
    }
}

pub fn set_func(
    scope: &mut v8::HandleScope<'_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    callback: v8::FunctionCallback,
) {
    let key = v8::String::new(scope, name).unwrap();
    let val = v8::Function::builder_raw(callback).build(scope).unwrap();
    val.set_name(key);
    obj.set(scope, key.into(), val.into());
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use anyhow::Error;
use lazy_static::lazy_static;
use crate::{ExternalFunctions, log};
use crate::modules::modules;
use crate::state::JSRunnerState;

lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub struct JSRunner {
    isolate: v8::OwnedIsolate
}

impl JSRunner {
    pub fn new(platform: Option<v8::SharedRef<v8::Platform>>, params: v8::CreateParams,
               externals: &ExternalFunctions, logger: *const ()) -> Result<Self, Error> {
        {
            let mut init = INITIALIZED.lock().expect("Couldn't unwrap muted");
            log(logger, format!("Initializing Serenity with mutex {}", init).as_str());
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

            let global = context.global(scope);

            let context_scope = &mut v8::ContextScope::new(scope, context);

            let modules = modules();

            for module in modules {
                if !&externals.modules.contains(&module.name) {
                    continue;
                }

                for (name, callback) in module.functions {
                    if name.contains('.') {
                        let split = name.find('.').unwrap();

                        let object = JSRunner::get_object(&name[0..split], context_scope, global);
                        JSRunner::set_func(context_scope, object, &name[split+1..], callback);
                    } else {
                        JSRunner::set_func(context_scope, global, name, callback)
                    }
                }

                for object_name in &externals.modules {
                    JSRunner::get_object(object_name.as_str(), context_scope, global);
                }
            }

            global_context = v8::Global::new(context_scope, context);
        }

        isolate.set_slot(Rc::new(RefCell::new(JSRunnerState {
            global_context,
            output: logger,
            external_functions: externals.function.clone(),
            id: externals.machine_id
        })));

        return Ok(JSRunner {
            isolate
        });
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
                return Err(Error::msg(v8::Exception::create_message(scope, exception).get(scope).to_rust_string_lossy(scope)));
            }
        };

        match script.run(try_catch) {
            Some(result) => Ok(result),
            None => {
                let exception = try_catch.exception().unwrap();
                let scope = &mut v8::HandleScope::new(try_catch);
                return Err(Error::msg(v8::Exception::create_message(scope, exception).get(scope).to_rust_string_lossy(scope)))
            }
        }
    }

    /// Initializes V8 engine
    fn initialize(platform: Option<v8::SharedRef<v8::Platform>>) {
        // Include 10MB ICU data file.
        #[repr(C, align(16))]
        struct IcuData([u8; 10454784]);
        static ICU_DATA: IcuData = IcuData(*include_bytes!("../icudtl.dat"));
        v8::icu::set_common_data_71(&ICU_DATA.0).unwrap();

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
    pub fn get_state(isolate: &v8::Isolate) -> Rc<RefCell<JSRunnerState>> {
        let s = isolate.get_slot::<Rc<RefCell<JSRunnerState>>>().unwrap();
        s.clone()
    }

    /// Returns the handle scope
    pub fn handle_scope(&mut self) -> v8::HandleScope {
        let context = self.global_context();
        v8::HandleScope::with_context(&mut self.isolate, context)
    }

    pub fn log(scope: &v8::HandleScope, message: &str) {
        let state = JSRunner::get_state(scope);
        let state = RefCell::borrow_mut(&state);
        let function: fn(*const u32, usize) = unsafe { std::mem::transmute(state.output) };
        (function)(message as *const str as *const u32, message.len());
    }

    pub fn get_object<'a>(name: &str, context_scope: &mut v8::ContextScope<'_, v8::HandleScope<'a>>, global: v8::Local<v8::Object>) -> v8::Local<'a, v8::Object> {
        let global_key = v8::String::new(context_scope, name).unwrap().into();

        return match global.get(context_scope, global_key) {
            Some(found) => {
                match found.try_into() {
                    Ok(found) => found,
                    Err(_error) => v8::Object::new(context_scope)
                }
            },
            None => {
                let object = v8::Object::new(context_scope);
                global.set(context_scope, global_key.into(), object.into());

                object
            }
        };
    }

    pub fn set_func(scope: &mut v8::HandleScope<'_>,
                    obj: v8::Local<v8::Object>,
                    name: &'static str,
                    function: v8::FunctionCallback) {
        let key = v8::String::new(scope, name).unwrap();
        let val = v8::Function::builder_raw(function).build(scope).unwrap();
        val.set_name(key);
        obj.set(scope, key.into(), val.into());
    }
}
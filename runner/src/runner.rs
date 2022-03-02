use std::cell::RefCell;
use std::rc::Rc;
use anyhow::Error;
use v8::CreateParams;
use util::error::custom_error;
use crate::bindings;

static INITIALIZED: bool = false;

struct JSRuntime {
    v8_isolate: Option<v8::OwnedIsolate>
}

impl JSRuntime {

    pub fn new(params: CreateParams) -> JSRuntime {
        let isolate = v8::Isolate::new(params);

        JSRuntime {
            v8_isolate: Some(isolate)
        }
    }

    /// Executes traditional JavaScript code (traditional = not ES modules).
    ///
    /// The execution takes place on the current global context, so it is possible
    /// to maintain local JS state and invoke this method multiple times.
    ///
    /// `name` can be a filepath or any other string, eg.
    ///
    ///   - "/some/file/path.js"
    ///   - "<anon>"
    ///   - "[native code]"
    ///
    /// The same `name` value can be used for multiple executions.
    ///
    /// `Error` can be downcast to a type that exposes additional information
    /// about the V8 exception. By default this type is `JsError`, however it may
    /// be a different type if `RuntimeOptions::js_error_create_fn` has been set.
    pub fn execute(&mut self, name: &str, source: &str) -> Result<v8::Global<v8::Value>, Error> {
        if !INITIALIZED {
            return Err(custom_error("RuntimeNotInitialized",
                                    "Tried to execute script before runtime was initialized"));
        }
        let scope = &mut self.handle_scope();

        let source = v8::String::new(scope, source).unwrap();
        let name = v8::String::new(scope, name).unwrap();
        let origin = bindings::script_origin(scope, name);

        let error_handler = &mut v8::TryCatch::new(scope);

        let script = match v8::Script::compile(error_handler, source, Some(&origin)) {
            Some(script) => script,
            None => {
                let exception = error_handler.exception().unwrap();
                return exception_to_err_result(error_handler, exception, false);
            }
        };

        match script.run(error_handler) {
            Some(value) => {
                let value_handle = v8::Global::new(error_handler, value);
                Ok(value_handle)
            }
            None => {
                assert!(error_handler.has_caught());
                let exception = error_handler.exception().unwrap();
                exception_to_err_result(error_handler, exception, false)
            }
        }
    }

    /// Creates a global HandleScope for the runner
    pub fn handle_scope(&mut self) -> v8::HandleScope {
        let context = self.global_context();
        v8::HandleScope::with_context(self.v8_isolate(), context)
    }

    /// Creates a global context
    pub fn global_context(&mut self) -> v8::Global<v8::Context> {
        let state = Self::state(self.v8_isolate());
        let state = state.borrow();
        state.global_context.clone().unwrap()
    }

    /// Creates a state from an isolate
    pub(crate) fn state(isolate: &v8::Isolate) -> Rc<RefCell<JsRuntimeState>> {
        let s = isolate.get_slot::<Rc<RefCell<JsRuntimeState>>>().unwrap();
        s.clone()
    }

    /// Gets the runner's isolate
    pub fn v8_isolate(&mut self) -> &mut v8::OwnedIsolate {
        self.v8_isolate.as_mut().unwrap()
    }
}

/// Converts a V8 exception to a Rust erroring Result with context
pub(crate) fn exception_to_err_result<'s, T>(
    scope: &mut v8::HandleScope<'s>,
    exception: v8::Local<v8::Value>,
    in_promise: bool) -> Result<T, Error> {
    let is_terminating_exception = scope.is_execution_terminating();
    let mut exception = exception;

    if is_terminating_exception {
        // TerminateExecution was called. Cancel exception termination so that the
        // exception can be created..
        scope.cancel_terminate_execution();

        // Maybe make a new exception object.
        if exception.is_null_or_undefined() {
            let message = v8::String::new(scope, "execution terminated").unwrap();
            exception = v8::Exception::error(scope, message);
        }
    }

    let mut js_error = JsError::from_v8_exception(scope, exception);
    if in_promise {
        js_error.message = format!(
            "Uncaught (in promise) {}",
            js_error.message.trim_start_matches("Uncaught ")
        );
    }

    let state_rc = self::state(scope);
    let state = state_rc.borrow();
    let js_error = (state.js_error_create_fn)(js_error);

    if is_terminating_exception {
        // Re-enable exception termination.
        scope.terminate_execution();
    }

    Err(js_error)
}

/// Initializes the V8 engine using the given platform (if one is given),
/// if not uses the default platform.
fn initialize(platform: Option<v8::SharedRef<v8::Platform>>) {
    // Include 10MB ICU data file.
    #[repr(C, align(16))]
    struct IcuData([u8; 10144432]);
    static ICU_DATA: IcuData = IcuData(*include_bytes!("icudtl.dat"));
    v8::icu::set_common_data_69(&ICU_DATA.0).unwrap();

    let platform = platform
        .unwrap_or_else(|| v8::new_default_platform(0, false).make_shared());
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let flags = concat!(
    " --harmony-import-assertions",
    " --no-validate-asm",
    );
    v8::V8::set_flags_from_string(flags);
}
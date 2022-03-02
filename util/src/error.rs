use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};
use anyhow::Error;

/// Creates a new error with a caller-specified error class name and message.
pub fn custom_error(
    class: &'static str,
    message: impl Into<Cow<'static, str>>,
) -> Error {
    CustomError {
        class,
        message: message.into(),
    }
        .into()
}

/// A simple error type that lets the creator specify both the error message and
/// the error class name. This type is private; externally it only ever appears
/// wrapped in an `anyhow::Error`. To retrieve the error class name from a wrapped
/// `CustomError`, use the function `get_custom_error_class()`.
#[derive(Debug)]
struct CustomError {
    class: &'static str,
    message: Cow<'static, str>,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CustomError {}

/// If this error was crated with `custom_error()`, return the specified error
/// class name. In all other cases this function returns `None`.
pub fn get_custom_error_class(error: &Error) -> Option<&'static str> {
    error.downcast_ref::<CustomError>().map(|e| e.class)
}

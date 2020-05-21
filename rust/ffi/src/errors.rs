use std::any::Any;
use std::cell::RefCell;
use std::error;
use std::ffi::CString;
use std::fmt;
use std::os::raw::c_char;
use std::panic::{self, UnwindSafe};
use std::ptr;

thread_local! {
    static LAST_ERROR: RefCell<Option<Box<dyn error::Error>>> = RefCell::new(None);
}
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parse(pest::error::Error<lexibook::wgl::Rule>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Error::IO(e) => e.to_string(),
            Error::Parse(e) => e.to_string(),
        };
        write!(f, "{}", string)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::IO(e) => e.source(),
            Error::Parse(e) => e.source(),
        }
    }
}

/// # Safety
/// Return the last error message
#[no_mangle]
pub unsafe extern "C" fn lexibook_last_error_message() -> *mut c_char {
    let last_error = match get_last_error() {
        Some(err) => err,
        None => return ptr::null_mut(),
    };

    let error_message = CString::new(last_error.to_string()).unwrap();
    error_message.into_raw()
}

fn get_last_error() -> Option<Box<dyn error::Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

pub fn update_last_error<E: Into<Box<dyn error::Error>> + 'static>(e: E) {
    let boxed = e.into();

    LAST_ERROR.with(|last| {
        *last.borrow_mut() = Some(boxed);
    });
}

pub fn catch_panic<T, E, F>(func: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E> + UnwindSafe,
    E: From<Box<dyn Any + Send + 'static>>,
{
    panic::catch_unwind(func)
        .map_err(Into::into)
        .and_then(|t| t)
}

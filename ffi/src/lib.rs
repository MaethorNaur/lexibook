#![allow(non_snake_case)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;
extern crate pest;
extern crate simple_logger;

use lexibook::sound_system;
use lexibook::sound_system::{MonoSyllableRepartition, SoundSystem};
use lexibook::wgl::Rule;
use log::Level;
use pest::error::Error;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;
use std::ptr;
use std::sync::Mutex;

lazy_static! {
    static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Warn);
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn to_level(self) -> Level {
        match self {
            LogLevel::Trace => Level::Trace,
            LogLevel::Debug => Level::Debug,
            LogLevel::Info => Level::Info,
            LogLevel::Warn => Level::Warn,
            LogLevel::Error => Level::Error,
        }
    }
}

#[repr(C)]
pub struct Words {
    words: *mut *mut c_char,
    len: usize,
    cap: usize,
}
#[no_mangle]
pub extern "C" fn lexibook_set_log_level(level: LogLevel) {
    *LOG_LEVEL.lock().unwrap() = level;
}

#[no_mangle]
pub extern "C" fn lexibook_free_words(words: Words) {
    unsafe {
        let mut v: Vec<*mut c_char> = Vec::from_raw_parts(words.words, words.len, words.cap);
        for ptr in &v {
            let _ = CString::from_raw(*ptr);
        }
        let s = v.as_mut_ptr();
        Box::from_raw(s);
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn lexibook_generate_words_from_file(
    file: *const c_char,
    number_of_words: u32,
    repartition: MonoSyllableRepartition,
) -> Words {
    simple_logger::init_with_level((*LOG_LEVEL.lock().unwrap()).to_level()).unwrap();
    let filename = unsafe {
        assert!(!file.is_null());
        CStr::from_ptr(file).to_str().unwrap()
    };
    let input = Box::leak(fs::read_to_string(filename).unwrap().into_boxed_str());
    match sound_system::from_string(input) {
        Ok(sound_system) => generated_words(sound_system, number_of_words, repartition),
        Err(e) => handle_error(e),
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn lexibook_generate_words_from_string(
    input_c_char: *const c_char,
    number_of_words: u32,
    repartition: MonoSyllableRepartition,
) -> Words {
    simple_logger::init_with_level((*LOG_LEVEL.lock().unwrap()).to_level()).unwrap();
    let input = unsafe {
        assert!(!input_c_char.is_null());
        CStr::from_ptr(input_c_char).to_str().unwrap()
    };
    match sound_system::from_string(input) {
        Ok(sound_system) => generated_words(sound_system, number_of_words, repartition),
        Err(e) => handle_error(e),
    }
}
fn generated_words(
    sound_system: SoundSystem,
    number_of_words: u32,
    repartition: MonoSyllableRepartition,
) -> Words {
    let generated_words = sound_system.generate_words(number_of_words as usize, repartition);
    let mut words: Vec<*mut c_char> = generated_words
        .into_iter()
        .map(|x| CString::new(x).unwrap().into_raw())
        .collect();
    let data = words.as_mut_ptr();
    let len = words.len();
    let cap = words.capacity();
    std::mem::forget(words);
    Words {
        words: data,
        len,
        cap,
    }
}
fn handle_error(error: Error<Rule>) -> Words {
    error!("{}", error);
    Words {
        words: ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

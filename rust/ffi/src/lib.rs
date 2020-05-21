#![allow(non_snake_case)]
extern crate lexibook;
extern crate log;

extern crate pest;
extern crate simple_logger;
pub mod errors;
mod types;

use errors::Error;
use lexibook::sound_system;
use lexibook::sound_system::SoundSystem;
use std::convert::{From, Into};
use std::ffi::CStr;
use std::fs;
use std::os::raw::{c_char, c_void};
use std::ptr;
use types::*;

#[no_mangle]
pub extern "C" fn lexibook_init_logger(level: LogLevel) {
    simple_logger::init_with_level(level.to_level()).unwrap();
}

#[no_mangle]
pub extern "C" fn lexibook_sound_system_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr as *mut SoundSystem);
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn lexibook_parse_string(input_c_char: *const c_char) -> *mut c_void {
    let input = unsafe {
        assert!(!input_c_char.is_null());
        CStr::from_ptr(input_c_char).to_str().unwrap()
    };
    match sound_system::from_string(input) {
        Ok(sound_system) => Box::into_raw(Box::new(sound_system)) as *mut c_void,
        Err(e) => {
            errors::update_last_error(e);
            ptr::null_mut()
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn lexibook_parse_file(file: *const c_char) -> *mut c_void {
    let filename = unsafe {
        assert!(!file.is_null());
        CStr::from_ptr(file).to_str().unwrap()
    };
    let result = fs::read_to_string(filename)
        .map_err(Error::IO)
        .and_then(|file| {
            let input = Box::leak(file.into_boxed_str());
            sound_system::from_string(input).map_err(Error::Parse)
        })
        .map(|sound_system| Box::into_raw(Box::new(sound_system)) as *mut c_void);

    match result {
        Ok(sound_system) => sound_system,
        Err(e) => {
            errors::update_last_error(e);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn lexibook_generate_words(
    ptr: *mut c_void,
    number_of_words: u32,
    repartition: MonoSyllableRepartition,
) -> *mut StringList {
    let sound_system = unsafe {
        assert!(!ptr.is_null());
        &mut *(ptr as *mut SoundSystem)
    };
    let generated_words = sound_system.generate_words(number_of_words as usize, repartition.into());
    Box::into_raw(Box::new(StringList::from(generated_words)))
}

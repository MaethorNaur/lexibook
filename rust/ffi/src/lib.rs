#![allow(non_snake_case)]
extern crate lexibook;
#[macro_use]
extern crate log;

extern crate pest;
extern crate simple_logger;
pub mod errors;
pub mod io;
pub mod sound_system;
mod types;

use lexibook::sound_system::SoundSystem;
use std::convert::{From, Into};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use types::*;

/// Initialise the logger
#[no_mangle]
pub extern "C" fn lexibook_init_logger(level: LogLevel) {
    simple_logger::SimpleLogger::new()
        .with_level(level.to_level())
        .init()
        .unwrap();
}

/// Free sound system memory
#[no_mangle]
pub extern "C" fn lexibook_sound_system_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr as *mut SoundSystem);
    }
}

/// Generate words from a sound system
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

/// Generate words from a sound system
#[no_mangle]
pub extern "C" fn lexibook_apply_transformations(
    ptr: *mut c_void,
    ptr_string_list: *mut StringList,
) -> *mut StringList {
    let sound_system = unsafe {
        assert!(!ptr.is_null());
        &mut *(ptr as *mut SoundSystem)
    };
    let string_list = unsafe {
        assert!(!ptr_string_list.is_null());
        &mut *ptr_string_list
    };
    let words: Vec<String> = string_list.to_vec();
    let transformations = sound_system.sound_trasformation(words);
    Box::into_raw(Box::new(StringList::from(transformations.output)))
}

/// Generate words from a sound system
#[no_mangle]
pub extern "C" fn lexibook_get_ipa(ptr: *mut c_void, word_ptr: *const c_char) -> *mut c_char {
    let sound_system = unsafe {
        assert!(!ptr.is_null());
        &mut *(ptr as *mut SoundSystem)
    };
    let word = unsafe {
        assert!(!word_ptr.is_null());
        CStr::from_ptr(word_ptr).to_str().unwrap()
    };
    let ipa = sound_system.ipa_representation(word);
    let result = CString::new(ipa).unwrap();
    result.into_raw()
}

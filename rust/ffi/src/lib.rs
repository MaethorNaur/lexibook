#![allow(non_snake_case)]
extern crate lexibook;
extern crate log;

extern crate pest;
extern crate simple_logger;
pub mod errors;
pub mod io;
pub mod sound_system;
mod types;

use lexibook::sound_system::SoundSystem;
use std::convert::{From, Into};
use std::os::raw::c_void;
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

use crate::errors;
use crate::errors::Error;
use lexibook::sound_system;
use lexibook::sound_system::SoundSystem;
use std::ffi::CStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::os::raw::{c_char, c_void};
use std::ptr;

/// Load a sound system from a string
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

/// Load a sound system from a file
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

/// Save a sound system to a file
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn lexibook_sound_system_save_file(ptr: *mut c_void, file: *const c_char) -> u8 {
    let sound_system = unsafe {
        assert!(!ptr.is_null());
        &mut *(ptr as *mut SoundSystem)
    };
    let filename = unsafe {
        assert!(!file.is_null());
        CStr::from_ptr(file).to_str().unwrap()
    };
    let wgl = sound_system.to_string();
    let result = File::create(filename).and_then(|mut file| file.write_all(wgl.as_bytes()));
    match result {
        Err(e) => {
            errors::update_last_error(e);
            0
        }
        _ => 1,
    }
}

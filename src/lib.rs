#![allow(non_snake_case)]

#[macro_use]
extern crate nom;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate zip;
mod sounds;
mod storage;
mod wgl;
use std::ffi::CStr;
use std::mem::transmute;
use std::os::raw::c_char;

use env_logger::Env;

#[no_mangle]
pub extern "C" fn lexibook_parse_file(file: *const c_char) -> u8 {
    env_logger::from_env(Env::default().default_filter_or("warn")).init();
    let filename = unsafe {
        assert!(!file.is_null());
        CStr::from_ptr(file).to_str().unwrap()
    };
    let res = wgl::from_file(filename);
    match res {
        Ok(o) => {
            println!("{:#?}", o);
            0
        }
        Err(wgl::Error::IO(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            error!("{} does not exist", filename);
            1
        }
        Err(wgl::Error::UnknownLetter(letter, class)) => {
            error!("Using an undefined {}, in {}", letter, class);
            1
        }
        Err(wgl::Error::Parse(nom::Err::Error(nom::Context::Code(i, e)))) => {
            error!("Error at {:?}, reason: {:#?}", i.0, e.description());
            1
        }
        Err(e) => {
            error!("{:#?}", e);
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn lexibook_save(file: *const c_char, txt: *const c_char) -> u8 {
    let filename = unsafe {
        assert!(!file.is_null());
        CStr::from_ptr(file).to_str().unwrap()
    };
    let text = unsafe {
        assert!(!txt.is_null());
        CStr::from_ptr(txt).to_str().unwrap()
    };
    match storage::write(filename, text) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

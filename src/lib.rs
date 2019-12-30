#![allow(non_snake_case)]

extern crate pest;
extern crate rand;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate log;
extern crate env_logger;

mod sound_system;
mod wgl;
use sound_system::MonoSyllableRepartition;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use env_logger::Env;

#[repr(C)]
pub struct Words {
    words: *mut *mut c_char,
    len: usize,
    cap: usize,
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

fn generated_words(
    ast: wgl::AST,
    number_of_words: u32,
    repartition: MonoSyllableRepartition,
) -> Words {
    let sound_system = sound_system::SoundSystem::compile(ast);
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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn lexibook_generate_words_from_file(
    file: *const c_char,
    number_of_words: u32,
    repartition: MonoSyllableRepartition,
) -> Words {
    env_logger::init_from_env(Env::default().default_filter_or("warn"));
    let filename = unsafe {
        assert!(!file.is_null());
        CStr::from_ptr(file).to_str().unwrap()
    };
    match wgl::from_file(filename) {
        Ok(ast) => generated_words(ast, number_of_words, repartition),
        Err(wgl::Error::IO(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            error!("{} does not exist", filename);
            Words {
                words: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
        Err(wgl::Error::Parse(e)) => {
            error!("{}", e);
            Words {
                words: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
        Err(wgl::Error::IO(ref e)) => {
            error!("{}", e);
            Words {
                words: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn lexibook_generate_words_from_string(
    input_c_char: *const c_char,
    number_of_words: u32,
    repartition: MonoSyllableRepartition,
) -> Words {
    env_logger::init_from_env(Env::default().default_filter_or("warn"));
    let input = unsafe {
        assert!(!input_c_char.is_null());
        CStr::from_ptr(input_c_char).to_str().unwrap()
    };
    match wgl::from_string(input) {
        Ok(ast) => generated_words(ast, number_of_words, repartition),
        Err(wgl::Error::Parse(e)) => {
            error!("{}", e);
            Words {
                words: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
        Err(wgl::Error::IO(ref e)) => {
            error!("{}", e);
            Words {
                words: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
    }
}

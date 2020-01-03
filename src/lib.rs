#![allow(non_snake_case)]
#[macro_use]
extern crate serde_derive;
extern crate cfg_if;
extern crate regex;

extern crate pest;
extern crate rand;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate log;

mod sound_system;
mod wgl;

cfg_if::cfg_if! {
    if #[cfg(feature = "wasm")] {
mod wasm;
pub use wasm::*;
    } else {
#[macro_use]
extern crate lazy_static;
pub mod ffi;
pub use ffi::*;
    }
}

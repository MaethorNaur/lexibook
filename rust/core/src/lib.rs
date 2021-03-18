#![allow(non_snake_case)]
#[macro_use]
extern crate serde_derive;
extern crate brotli;
extern crate pest;
extern crate rand;
extern crate regex;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate log;

mod glossary;
pub mod sound_system;
pub mod wgl;
pub use glossary::Glossary;

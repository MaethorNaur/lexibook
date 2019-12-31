#![allow(non_snake_case)]

extern crate regex;

extern crate pest;
extern crate rand;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate log;

mod sound_system;
mod wgl;
pub use sound_system::{phone, MonoSyllableRepartition, SoundSystem, Transformation};
pub use wgl::{Error, Rule};

pub fn from_file(filename: &'_ str) -> Result<SoundSystem<'_>, Error<wgl::Rule>> {
    wgl::from_file(filename).map(SoundSystem::compile)
}
pub fn from_string(input: &'_ str) -> Result<SoundSystem<'_>, Error<wgl::Rule>> {
    wgl::from_string(input).map(SoundSystem::compile)
}

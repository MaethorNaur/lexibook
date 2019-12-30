#[macro_use]
extern crate prettytable;

use prettytable::Table;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate pest;
extern crate rand;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate clap;

use clap::App;
use env_logger::Env;

mod sound_system;
mod wgl;
use wgl::Error;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("warn"));
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let filename = matches.value_of("FILE").unwrap();
    let numbers = value_t!(matches, "numbers", usize).unwrap_or(10);
    let repartition = value_t!(matches, "syllable", sound_system::MonoSyllableRepartition)
        .unwrap_or(sound_system::MonoSyllableRepartition::LessFrequent);
    let maybe_output = matches.value_of("output");
    match wgl::from_file(filename) {
        Ok(ast) => {
            let sound_system = sound_system::SoundSystem::compile(ast);
            let words = sound_system.generate_words(numbers, repartition);
            match maybe_output {
                None => {
                    let mut table = Table::new();
                    table.add_row(row!["Word"]);
                    words.iter().for_each(|word| {
                        table.add_row(row![word]);
                    });
                    table.printstd();
                }
                Some(output) => {
                    let path = Path::new(output);
                    let mut file = File::create(&path).unwrap();
                    file.write_all(words.join("\n").as_bytes()).unwrap();
                }
            }
        }
        Err(Error::IO(error)) => error!("{}", error),
        Err(Error::Parse(error)) => error!("{}", error),
    }
}

#[macro_use]
extern crate serde_derive;

extern crate cfg_if;
extern crate prettytable;
extern crate regex;

extern crate pest;
extern crate rand;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

#[macro_use]
extern crate clap;
mod sound_system;
mod wgl;

use clap::App;
use log::Level;
use prettytable::{color, Attr, Cell, Row, Table};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use sound_system::rules;
use sound_system::MonoSyllableRepartition;

pub fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let filename = matches.value_of("FILE").unwrap();

    let level = match matches.occurrences_of("debug") {
        0 => Level::Warn,
        1 => Level::Debug,
        _ => Level::Trace,
    };
    simple_logger::init_with_level(level).unwrap();

    let skip_transformation = matches.is_present("skip_transformation");

    let numbers = value_t!(matches, "numbers", usize).unwrap_or(10);
    let repartition = value_t!(matches, "syllable", MonoSyllableRepartition)
        .unwrap_or(MonoSyllableRepartition::LessFrequent);
    let maybe_output = matches.value_of("output");
    let input = Box::leak(fs::read_to_string(filename).unwrap().into_boxed_str());
    match sound_system::from_string(input) {
        Ok(sound_system) => {
            let words = sound_system.generate_words(numbers, repartition);
            let transformations = if skip_transformation {
                Default::default()
            } else {
                rules::sound_trasformation(&sound_system, words.clone())
            };
            debug!("transformation rules {:#?}", transformations);
            match maybe_output {
                None => {
                    let mut table = Table::new();
                    let mut header = vec![Cell::new("Generated Word")
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::CYAN))];

                    let rules = transformations
                        .history
                        .iter()
                        .map(|t| &t.0)
                        .collect::<Vec<_>>();
                    rules.iter().for_each(|rule| {
                        header.push(
                            Cell::new(rule)
                                .with_style(Attr::Bold)
                                .with_style(Attr::ForegroundColor(color::BRIGHT_RED)),
                        )
                    });
                    if !rules.is_empty() {
                        header.push(
                            Cell::new("Final word")
                                .with_style(Attr::Bold)
                                .with_style(Attr::ForegroundColor(color::CYAN)),
                        );
                    }
                    table.add_row(Row::new(header));
                    words.iter().enumerate().for_each(|(i, word)| {
                        let mut row = vec![Cell::new(word)];
                        rules.iter().enumerate().for_each(|(rule, _)| {
                            if let Some((_, history)) = transformations.history.get(rule) {
                                let previous = if rule == 0 {
                                    word
                                } else {
                                    let (_, previous_history) = &transformations.history[rule - 1];
                                    &previous_history[i]
                                };
                                let current = &history[i];
                                let to_add = if current == previous {
                                    Cell::new("--")
                                } else {
                                    Cell::new(current)
                                        .with_style(Attr::Bold)
                                        .with_style(Attr::ForegroundColor(color::RED))
                                };
                                row.push(to_add);
                            }
                        });
                        if !transformations.output.is_empty() {
                            row.push(Cell::new(&transformations.output[i]).with_style(Attr::Bold));
                        }
                        table.add_row(Row::new(row));
                    });
                    table.printstd();
                }
                Some(output) => {
                    let path = Path::new(output);
                    let mut file = File::create(&path).unwrap();
                    let bytes = if skip_transformation {
                        words
                    } else {
                        transformations.output
                    };
                    file.write_all(bytes.join("\n").as_bytes()).unwrap();
                }
            }
        }
        Err(error) => error!("{}", error),
    }
}

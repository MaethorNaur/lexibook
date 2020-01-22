extern crate prettytable;
#[macro_use]
extern crate log;
extern crate fern;
#[macro_use]
extern crate clap;
extern crate csv;
extern crate pest;
mod errors;
mod output;
use clap::{App, ArgMatches};
use errors::*;
use fern::colors::{Color, ColoredLevelConfig};
use lexibook::sound_system::rules::Transformation;
use lexibook::sound_system::{MonoSyllableRepartition, SoundSystem};
use log::LevelFilter;
use std::convert::From;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn setup_log<'a>(matches: &ArgMatches<'a>) {
    if !matches.is_present("silent") {
        let level = match matches.occurrences_of("verbose") {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        fern::Dispatch::new()
            .format(move |out, message, record| {
                use log::Level::*;
                let symbol = match record.level() {
                    Trace => "◯",
                    Debug => "◎",
                    Info => "●",
                    Warn => "⌿",
                    Error => "✖",
                };
                let colors = ColoredLevelConfig::new()
                    .trace(Color::Magenta)
                    .debug(Color::Blue)
                    .info(Color::Green)
                    .warn(Color::Yellow)
                    .error(Color::Red);

                let target = format!(
                    "\x1B[{}m{}\x1B[0m",
                    Color::BrightBlack.to_fg_str(),
                    record.target()
                );

                out.finish(format_args!(
                    "{color_line}{} {: <width$}\x1B[0m {} > {}",
                    symbol,
                    colors.color(record.level()),
                    target,
                    message,
                    color_line =
                        format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                    width = 5
                ))
            })
            .level(level)
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    }
}

pub fn main() {
    let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);
    let matches = app.clone().get_matches();

    setup_log(&matches);
    let result = matches
        .subcommand_matches("words")
        .map(words)
        .or_else(|| matches.subcommand_matches("sounds").map(sounds))
        .or_else(|| matches.subcommand_matches("phonology").map(phonology))
        .unwrap_or_else(|| app.print_long_help().map_err(From::from));
    match result {
        Ok(_) => (),
        Err(error) => error!("{}", error),
    }
}

fn phonology<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    let filename = matches.value_of("FILE").unwrap();
    let input = Box::leak(fs::read_to_string(filename).unwrap().into_boxed_str());
    lexibook::sound_system::from_string(input)
        .map_err(From::from)
        .and_then(|sound_system| {
            let (consonants_table, vowel_table) = output::create_phonology_tables(sound_system);
            println!("Consonants\n");
            consonants_table.printstd();
            println!("\nVowels\n");
            vowel_table.printstd();
            Ok(())
        })
}

fn words<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    let filename = matches.value_of("FILE").unwrap();
    let skip_transformation = matches.is_present("skip_transformation");

    let numbers = value_t!(matches, "numbers", usize).unwrap_or(10);
    let repartition = value_t!(matches, "syllable", MonoSyllableRepartition)
        .unwrap_or(MonoSyllableRepartition::LessFrequent);
    let maybe_output = matches.value_of("output");
    let input = Box::leak(fs::read_to_string(filename).unwrap().into_boxed_str());

    lexibook::sound_system::from_string(input)
        .map_err(From::from)
        .and_then(|mut sound_system| {
            let words = sound_system.generate_words(numbers, repartition);
            let transformations = if skip_transformation {
                Transformation {
                    output: words.clone(),
                    ..Default::default()
                }
            } else {
                sound_system.sound_trasformation(words.clone())
            };
            pretty_print(
                matches.is_present("display_transformations"),
                &sound_system,
                words,
                transformations,
                maybe_output,
            )
        })
        .map_err(From::from)
}

fn sounds<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    let filename = matches.value_of("FILE").unwrap();
    let input = Box::leak(fs::read_to_string(filename).unwrap().into_boxed_str());

    let maybe_output = matches.value_of("output");
    lexibook::sound_system::from_string(input)
        .map_err(From::from)
        .and_then(|mut sound_system| {
            let stdin = io::stdin();
            let words: Result<Vec<String>> = match matches.value_of("INPUT") {
                Some(filename) => File::open(filename).map_err(From::from).map(|file| {
                    io::BufReader::new(file)
                        .lines()
                        .map(|ln| ln.unwrap())
                        .collect()
                }),
                None => Ok(stdin.lock().lines().map(|ln| ln.unwrap()).collect()),
            };

            words.and_then(|words| {
                let transformations = sound_system.sound_trasformation(words.clone());
                pretty_print(
                    matches.is_present("display_transformations"),
                    &sound_system,
                    words,
                    transformations,
                    maybe_output,
                )
            })
        })
}

fn pretty_print(
    pretty: bool,
    sound_system: &SoundSystem,
    words: Vec<String>,
    transformations: Transformation,
    maybe_output: Option<&str>,
) -> Result<()> {
    if pretty {
        let table = output::create_table(&sound_system, words, transformations);
        match maybe_output {
            Some(output) => output::csv(&table, output),
            None => output::stdout(&table),
        }
    } else {
        let stdout = io::stdout();
        let mut writer: Box<dyn Write> = match maybe_output {
            Some(output) => {
                let path = Path::new(output);
                Box::new(File::create(&path).unwrap())
            }
            None => Box::new(stdout.lock()),
        };
        writer
            .write_all(transformations.output.join("\n").as_bytes())
            .and_then(|_| writer.write("\n".as_bytes()))
            .map(|_| ())
            .map_err(From::from)
    }
}

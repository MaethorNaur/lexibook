extern crate prettytable;
#[macro_use]
extern crate log;
extern crate clap_verbosity_flag;
extern crate csv;
extern crate fern;
extern crate pest;

mod cli;
mod errors;
mod output;
use cli::*;
use errors::*;
use fern::colors::{Color, ColoredLevelConfig};
use lexibook::sound_system::rules::Transformation;
use lexibook::sound_system::SoundSystem;
use std::convert::From;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use structopt::StructOpt;

fn setup_log(verbosity: Option<log::Level>) {
    if let Some(level) = verbosity {
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
            .level(level.to_level_filter())
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    }
}

pub fn main() {
    let opt = Cli::from_args();
    setup_log(opt.verbosity());
    let result = match opt {
        Cli::Phonology(command) => phonology(command.filename),
        Cli::Words(command) => words(command),
        Cli::Sounds(command) => sounds(command),
    };

    match result {
        Ok(_) => (),
        Err(error) => error!("{}", error),
    }
}

fn phonology(filename: PathBuf) -> Result<()> {
    let input = Box::leak(fs::read_to_string(filename).unwrap().into_boxed_str());
    lexibook::sound_system::from_string(input)
        .map_err(From::from)
        .map(|sound_system| {
            let (consonants_table, vowel_table) = output::create_phonology_tables(sound_system);
            println!("Consonants\n");
            consonants_table.printstd();
            println!("\nVowels\n");
            vowel_table.printstd();
        })
}

fn words(command: Words) -> Result<()> {
    let input = Box::leak(
        fs::read_to_string(command.filename.as_path())
            .unwrap()
            .into_boxed_str(),
    );

    let numbers = command.numbers;
    let repartition = command.repartition;
    let pretty = command.common.pretty;
    let maybe_output = command.common.output;
    let skip_transformation = command.skip_transformation;

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
            pretty_print(pretty, &sound_system, words, transformations, maybe_output)
        })
        .map_err(From::from)
}

fn sounds(command: Sounds) -> Result<()> {
    let input = Box::leak(
        fs::read_to_string(command.filename.as_path())
            .unwrap()
            .into_boxed_str(),
    );

    let pretty = command.common.pretty;
    let maybe_output = command.common.output;
    let input_words = command.input;

    lexibook::sound_system::from_string(input)
        .map_err(From::from)
        .and_then(|mut sound_system| {
            let stdin = io::stdin();
            let words: Result<Vec<String>> = match input_words {
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
                pretty_print(pretty, &sound_system, words, transformations, maybe_output)
            })
        })
}

fn pretty_print(
    pretty: bool,
    sound_system: &SoundSystem,
    words: Vec<String>,
    transformations: Transformation,
    maybe_output: Option<PathBuf>,
) -> Result<()> {
    if pretty {
        let table = output::create_table(&sound_system, words, transformations);
        let maybe_output = maybe_output.as_ref().and_then(|p| p.to_str());
        match maybe_output {
            Some(output) => output::csv(&table, output),
            None => output::stdout(&table),
        }
    } else {
        let stdout = io::stdout();
        let mut writer: Box<dyn Write> = match maybe_output {
            Some(path) => Box::new(File::create(path).unwrap()),
            None => Box::new(stdout.lock()),
        };
        writer
            .write_all(transformations.output.join("\n").as_bytes())
            .and_then(|_| writer.write(b"\n"))
            .map(|_| ())
            .map_err(From::from)
    }
}

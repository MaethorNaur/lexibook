mod ast;
mod errors;
mod parser;

use crate::sounds;
use ast::*;
pub use errors::Error;
use nom::types::CompleteStr;
pub use parser::*;
use std::fs;

fn fold_exprs(expressions: Vec<Expr>) -> Result<sounds::SoundSystem<'_>, Error> {
    let mut sound_system = sounds::SoundSystem::new();
    for expression in expressions {
        match expression {
            Expr::Letters(letters) => {
                for (value, string_sound) in letters {
                    sound_system.add(sounds::new_letter(value, string_sound))
                }
                sound_system.generate_default_classes()
            }
            Expr::Class(class, letters) => match sound_system.add_classes(class, letters) {
                Ok(()) => (),
                Err(letter) => return Err(Error::from((letter, class))),
            },
            Expr::Words(syllables) => {
                let res: Vec<Vec<sounds::Syllable>> = syllables
                    .into_iter()
                    .map(|vec| {
                        vec.into_iter()
                            .map(|syllable| match syllable {
                                Syllable::Class(c) => sounds::Syllable::Class(c),
                                Syllable::Optional => sounds::Syllable::Optional,
                                Syllable::Sound(s) => sounds::Syllable::Sounds(s),
                            })
                            .collect::<Vec<sounds::Syllable>>()
                    })
                    .collect();

                sound_system.syllables = res;
            }
            _ => (),
        }
    }

    Ok(sound_system)
}
pub fn from_string(input: &str) -> Result<sounds::SoundSystem<'_>, Error> {
    parser::do_parse(CompleteStr::from(input))
        .map_err(errors::Error::from)
        .and_then(|(_input, result)| fold_exprs(result))
}

pub fn from_file(filename: &str) -> Result<sounds::SoundSystem, Error> {
    fs::read_to_string(filename)
        .map_err(errors::Error::from)
        .and_then(|input| from_string(Box::leak(input.into_boxed_str())))
}

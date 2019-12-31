use super::distribution::frequency;
use super::phone::*;
use super::SoundSystem;
use crate::wgl::{Environment, Letter, TransformationRule, AST};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;

impl<'a> SoundSystem<'a> {
    pub fn compile(ast: AST<'a>) -> Self {
        let mut sound_system = SoundSystem::new();
        let letters: Vec<_> = ast.letters.iter().map(|s| s.to_string()).collect();
        sound_system.distribution = frequency(letters);
        ast.letters.iter().for_each(|letter| {
            let notation = convert_ipa_letters_to_sounds(letter);
            sound_system.add_phonemes(letter.to_string(), notation)
        });

        let classes = &mut sound_system.classes;
        for (repr, phones) in &sound_system.phonemes {
            match phones.iter().find(|phone| match phone {
                Phone::Vowel(_) | Phone::Consonant(_) => true,
                _ => false,
            }) {
                Some(Phone::Vowel(_)) => {
                    let vec = classes.entry("V").or_insert_with(|| vec![]);
                    vec.push(repr);
                }
                Some(Phone::Consonant(_)) => {
                    let vec = classes.entry("C").or_insert_with(|| vec![]);
                    vec.push(repr);
                }
                _ => {}
            }
        }
        for (class_name, letters) in ast.classes {
            classes.insert(class_name, letters);
        }
        sound_system.rules = ast
            .rules
            .iter()
            .map(|rule| (rule.to_string(), rule_to_regex(classes, rule), rule.output))
            .collect::<Vec<_>>();
        sound_system.syllables = ast.syllables.clone();
        debug!("Sound system compiled: {:#?}", sound_system);
        sound_system
    }
}

fn convert_ipa_letters_to_sounds(letter: &Letter<'_>) -> Vec<Phone> {
    let mut sounds = vec![];
    match letter {
        Letter::OnlyRepresensation(s) => sounds.push(Phone::try_from(*s).unwrap()),
        Letter::WithPhoneticNotation(_, notations) => notations
            .iter()
            .filter_map(|s| Phone::try_from(*s).ok())
            .for_each(|s| sounds.push(s)),
    }
    sounds
}

fn rule_to_regex(classes: &HashMap<&'_ str, Vec<&'_ str>>, rule: &TransformationRule<'_>) -> Regex {
    let mut regex = String::new();
    let mut input = String::from("(?P<input>");
    rule.input.chars().for_each(|c| match c {
        c if c.is_uppercase() => {
            let class_name = c.to_string();
            if let Some(letters) = classes.get(&class_name as &str) {
                input.push_str("(?P<");
                input.push(c);
                input.push('>');
                let choice = letters
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect::<Vec<_>>()
                    .join("|");
                input.push_str(&choice);
                input.push(')');
            };
        }
        c => input.push(c),
    });
    input.push(')');
    match rule.environment {
        Environment::All => regex.push_str(&input),
        Environment::Match(pattern) => pattern.chars().enumerate().for_each(|(index, c)| match c {
            '#' if index == 0 => regex.push('^'),
            '#' => regex.push('$'),
            class if class.is_uppercase() => {
                let class_name = c.to_string();
                if let Some(letters) = classes.get(&class_name as &str) {
                    regex.push('(');
                    let choice = letters
                        .iter()
                        .map(|s| (*s).to_string())
                        .collect::<Vec<_>>()
                        .join("|");
                    regex.push_str(&choice);
                    regex.push(')');
                };
            }
            '_' => regex.push_str(&input),
            '*' => regex.push_str(".*?"),
            letter => regex.push(letter),
        }),
    }
    Regex::new(&regex).unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_ipa_letters_to_sounds() {
        let letter = Letter::WithPhoneticNotation("a", vec!["a", "n"]);
        let result = convert_ipa_letters_to_sounds(&letter);
        assert_eq!(
            result,
            vec![
                Phone::Vowel(Vowel {
                    height: Height::Open,
                    backness: Backness::Front,
                    roundness: Roundness::UnRounded,
                    properties: vec![]
                }),
                Phone::Consonant(Consonant {
                    place: ConsonantPlace::Alveolar,
                    manner: ConsonantManner::Nasal,
                    properties: vec![PhoneProperty::Phonation(Phonation::Voiced)]
                })
            ]
        )
    }
}

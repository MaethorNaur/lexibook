use super::distribution::frequency;
use super::phone::*;
use super::{PhonemeRule, Rule, SoundRule, SoundSystem};
use crate::wgl::{Environment, Letter, TransformationRule, AST};
use std::collections::HashMap;
use std::convert::TryFrom;

impl SoundSystem {
    pub fn compile(ast: AST<'_>) -> Self {
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
                    let vec = classes.entry("V".to_string()).or_insert_with(|| vec![]);
                    vec.push(repr.to_string());
                }
                Some(Phone::Consonant(_)) => {
                    let vec = classes.entry("C".to_string()).or_insert_with(|| vec![]);
                    vec.push(repr.to_string());
                }
                _ => {}
            }
        }
        for (class_name, letters) in ast.classes {
            classes.insert(
                class_name.to_string(),
                letters.iter().map(|s| (*s).to_string()).collect::<Vec<_>>(),
            );
        }
        sound_system.rules = ast
            .rules
            .iter()
            .map(|rule| match rule {
                TransformationRule::SoundRule(_) => Rule::SoundRule(SoundRule {
                    name: rule.to_string(),
                    regex: rule_to_regex(classes, rule),
                    replacement: rule.output().map(|s| s.to_owned()),
                }),
                TransformationRule::PhonemeRule(_) => Rule::PhonemeRule(PhonemeRule {
                    name: rule.to_string(),
                    phoneme: rule.input().to_string(),
                    phones: rule
                        .output()
                        .map(|o| o.chars().filter_map(|c| Phone::try_from(c).ok()).collect())
                        .unwrap_or_else(|| vec![]),
                }),
            })
            .collect::<Vec<_>>();
        sound_system.syllables = ast
            .syllables
            .iter()
            .map(|l| l.iter().map(|s| (*s).to_string()).collect())
            .collect();
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

fn rule_to_regex(classes: &HashMap<String, Vec<String>>, rule: &TransformationRule<'_>) -> String {
    let mut regex = String::new();
    let mut input = String::from("(?P<input>");
    rule.input().chars().for_each(|c| match c {
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
    match rule.environment().unwrap() {
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
            '*' => regex.push_str("(.*?)"),
            letter => regex.push(letter),
        }),
    }
    regex
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
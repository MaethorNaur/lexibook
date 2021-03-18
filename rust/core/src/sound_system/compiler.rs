use super::distribution::frequency;
use super::phone::*;
use super::{Condition, PhonemeCondition, PhonemeDifference, Rule, SoundSystem};
use crate::wgl::{Ast, Environment, TransformationRule};
use std::collections::HashMap;
use std::convert::{Into, TryFrom};

impl SoundSystem {
    pub fn compile(ast: Ast) -> Self {
        let distribution = frequency(&ast.letters);
        let mut phonemes: HashMap<String, Vec<PhonemeCondition>> = ast
            .letters
            .iter()
            .map(|c| {
                (
                    c.letter.clone(),
                    vec![(convert_ipa_letters_to_sounds(&c.letter), Condition::Always)],
                )
            })
            .collect();
        ast.phonemes
            .iter()
            .flat_map(|(letter, list)| list.iter().map(move |tuple| (letter, tuple)))
            .for_each(|(letter, phoneme)| {
                let phones = phoneme
                    .notation
                    .chars()
                    .filter_map(|c| Phone::try_from(c).ok())
                    .collect::<Vec<_>>();
                let entry = phonemes
                    .entry((*letter).to_string())
                    .or_insert_with(Vec::new);
                let mut new = true;
                for (_, elem) in entry.iter_mut().enumerate() {
                    if elem.1 == phoneme.condition.clone().into() {
                        *elem = (phones.clone(), elem.1.clone());
                        new = false;
                        break;
                    }
                }
                if new {
                    entry.push((phones, phoneme.condition.clone().into()));
                }
            });

        let mut classes: HashMap<String, Vec<String>> = HashMap::new();

        let mut sorted_phonemes: Vec<_> = phonemes
            .iter()
            .filter(|(repr, _)| ast.letters.iter().any(|c| &c.letter == *repr))
            .collect::<Vec<_>>();
        sorted_phonemes
            .sort_unstable_by(|(_, left), (_, right)| Ord::cmp(&right[0].0[0], &left[0].0[0]));
        trace!("Sorted: {:#?}", sorted_phonemes);
        for (repr, vec) in &sorted_phonemes {
            if let Some(phone_classes) = vec.get(0).unwrap().0.get(0).unwrap().classes() {
                for classe in phone_classes {
                    let vec = classes.entry(classe.to_string()).or_insert_with(Vec::new);
                    vec.push((*repr).to_string());
                }
            }
        }

        for (class_name, letters) in ast.classes {
            classes.insert(
                class_name.to_string(),
                letters.iter().map(|s| (*s).to_string()).collect::<Vec<_>>(),
            );
        }

        let rules = ast
            .rules
            .iter()
            .map(|rule| match rule {
                TransformationRule::SoundRule { .. } => Rule::SoundRule {
                    name: rule.to_string(),
                    regex: rule_to_regex(&classes, rule),
                    replacement: rule.output().map(|s| s.to_string()),
                },
                TransformationRule::PhonemeRule { .. } => Rule::PhonemeRule {
                    name: rule.to_string(),
                    phoneme_differences: rule_to_phoneme_differences(
                        &classes,
                        &rule.input(),
                        rule.output(),
                    ),
                },
            })
            .collect::<Vec<_>>();
        let syllables = ast
            .syllables
            .iter()
            .map(|l| l.iter().map(|s| (*s).to_string()).collect())
            .collect();
        let sound_system = SoundSystem::new(classes, phonemes, syllables, distribution, rules);
        trace!("Sound system compiled: {:#?}", sound_system);
        sound_system
    }
}

fn rule_to_phoneme_differences(
    classes: &HashMap<String, Vec<String>>,
    phoneme: &'_ str,
    maybePhones: Option<String>,
) -> Vec<PhonemeDifference> {
    let expanded = expand(classes, phoneme);
    trace!("phonemes expanded to: {:#?}", expanded);
    match maybePhones {
        None => expanded
            .iter()
            .map(|phoneme| PhonemeDifference::Delete(phoneme.to_string()))
            .collect(),
        Some(phones) => {
            let expanded_phones = expand(classes, &phones);
            expanded
                .iter()
                .enumerate()
                .filter_map(|(idx, phoneme)| {
                    expanded_phones.get(idx).map(|phone_str| {
                        let phones = phone_str
                            .chars()
                            .filter_map(|c| Phone::try_from(c).ok())
                            .collect();
                        PhonemeDifference::Upsert(phoneme.to_string(), phones)
                    })
                })
                .collect()
        }
    }
}

fn expand(classes: &HashMap<String, Vec<String>>, s: &'_ str) -> Vec<String> {
    s.chars()
        .flat_map(|c| match c {
            'Ṽ' => vec!['V', '\u{303}'],
            'Ẽ' => vec!['E', '\u{303}'],
            'Ỹ' => vec!['Y', '\u{303}'],
            _ => vec![c],
        })
        .fold(vec![], |mut phonemes, c| {
            if c.is_uppercase() {
                match classes.get(&c.to_string()) {
                    None => phonemes,
                    Some(letters) => {
                        if phonemes.is_empty() {
                            letters.clone()
                        } else {
                            phonemes
                                .iter()
                                .flat_map(|s| letters.iter().map(move |l| format!("{}{}", s, l)))
                                .collect()
                        }
                    }
                }
            } else {
                if phonemes.is_empty() {
                    phonemes.push(c.to_string())
                } else {
                    phonemes.iter_mut().for_each(|string| string.push(c))
                };
                phonemes
            }
        })
}

fn convert_ipa_letters_to_sounds(letter: &str) -> Vec<Phone> {
    letter
        .chars()
        .filter_map(|c| Phone::try_from(c).ok())
        .collect()
}

fn rule_to_regex(classes: &HashMap<String, Vec<String>>, rule: &TransformationRule) -> String {
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
        let letters = "an";
        let result = convert_ipa_letters_to_sounds(letters);
        assert_eq!(
            result,
            vec![
                Phone::Vowel(Vowel {
                    height: Height::Open,
                    backness: Backness::Front,
                    roundness: Roundness::UnRounded,
                }),
                Phone::Consonant(Consonant {
                    place: ConsonantPlace::Alveolar,
                    manner: ConsonantManner::Nasal,
                    phonation: Phonation::Voiced
                })
            ]
        )
    }
}

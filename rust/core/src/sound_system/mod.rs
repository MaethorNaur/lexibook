use crate::wgl;
use pest::error::Error;
use std::fmt;
mod compiler;
mod distribution;
mod generator;
mod types;

pub mod phone;
pub mod rules;
pub use compiler::*;
pub use types::*;

pub fn from_string(input: &'_ str) -> Result<SoundSystem, Error<wgl::Rule>> {
    wgl::from_string(input).map(SoundSystem::compile)
}

impl fmt::Display for SoundSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "letters: ")?;
        let letters = self
            .distribution()
            .iter()
            .map(|(letter, _)| letter.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(f, "{}", &letters)
    }
}

impl SoundSystem {
    fn add_phonemes(&mut self, repr: &'_ str, phones: phone::Phones) {
        self.phonemes()
            .insert(repr.to_string(), vec![(phones, Condition::Always)]);
    }

    pub fn update_phoneme(&mut self, diffs: &[PhonemeDifference]) {
        diffs.iter().for_each(|diff| match diff {
            PhonemeDifference::Skip => (),
            PhonemeDifference::Delete(repr) => {
                self.phonemes().remove(repr);
            }
            PhonemeDifference::Upsert(repr, phones) => self.add_phonemes(repr, phones.clone()),
        });
        self.sort_phonemes()
    }

    pub fn phonology(&self) -> phone::Phones {
        let mut vec: phone::Phones = self
            .phonemes_sorted()
            .clone()
            .into_iter()
            .flat_map(|(_, list)| list.into_iter().flat_map(|(phones, _)| phones))
            .filter(|phone| match phone {
                phone::Phone::Diacritic(_) => false,
                _ => true,
            })
            .collect();
        vec.sort_unstable_by(|left_phones, right_phones| Ord::cmp(&right_phones, &left_phones));

        vec.dedup();
        vec
    }

    pub fn ipa_representation(&self, word: &'_ str) -> String {
        let mut result = String::new();
        let phonemes = self.phonemes_sorted();
        let mut input = word.to_string();
        let mut position: usize = 0;
        let length = word.len();
        while !input.is_empty() {
            let skip = match phonemes
                .iter()
                .find_map(|tuple| self.find_phoneme(&input, tuple, position, length))
            {
                Some((letter, phones)) => {
                    phones
                        .iter()
                        .map(|p| p.clone().into())
                        .for_each(|c| result.push(c));
                    letter.chars().count()
                }
                None => 1,
            };
            position += skip;
            input = input.chars().skip(skip).collect::<String>();
        }
        result
    }

    fn find_phoneme<'a, 'b>(
        &self,
        input: &'b str,
        letters_condition: &'a (String, Vec<PhonemeCondition>),
        position: usize,
        length: usize,
    ) -> Option<(&'a String, &'a phone::Phones)> {
        let (letter, list) = letters_condition;
        let mut vec = list.into_iter().collect::<Vec<_>>();
        vec.sort_unstable_by(|(_, left), (_, right)| Ord::cmp(&right, &left));
        vec.into_iter().find_map(|(phones, condition)| {
            let result = (letter, phones);
            if input.starts_with(letter)
                && self.resolve_condition(input, letter, position, length, condition)
            {
                Some(result)
            } else {
                None
            }
        })
    }

    fn resolve_condition(
        &self,
        input: &str,
        letter: &str,
        position: usize,
        length: usize,
        condition: &Condition,
    ) -> bool {
        match condition {
            Condition::Single(cond_type) => {
                self.resolve_condition_type(input, letter, position, length, cond_type)
            }
            Condition::Always => true,
            Condition::Not(cond_type) => {
                !self.resolve_condition_type(input, letter, position, length, cond_type)
            }
            Condition::Binary {
                operand,
                left,
                right,
            } => {
                let left_bool = self.resolve_condition(input, letter, position, length, left);
                let right_bool = self.resolve_condition(input, letter, position, length, right);
                match operand {
                    ConditionOperand::And => left_bool && right_bool,
                    ConditionOperand::Or => left_bool || right_bool,
                }
            }
        }
    }
    fn resolve_condition_type(
        &self,
        input: &str,
        letter: &str,
        position: usize,
        length: usize,
        condition: &ConditionType,
    ) -> bool {
        match condition {
            ConditionType::BeginningWord if position == 0 => true,
            ConditionType::EndWord if position + letter.len() == length => true,
            ConditionType::FollowedBy(value) => {
                value.chars().enumerate().fold(true, |is_valid, (i, c)| {
                    let value = c.to_string();
                    let next = if c.is_uppercase() {
                        self.classes()
                            .get(&value)
                            .map(|v| {
                                v.iter()
                                    .find_map(|s| {
                                        input
                                            .chars()
                                            .nth(position + i + 1)
                                            .map(|p| p.to_string().eq(s))
                                    })
                                    .unwrap_or(false)
                            })
                            .unwrap_or(false)
                    } else {
                        input
                            .chars()
                            .nth(position + i + 1)
                            .map(|p| p == c)
                            .unwrap_or(false)
                    };
                    is_valid && next
                })
            }
            _ => false,
        }
    }
}

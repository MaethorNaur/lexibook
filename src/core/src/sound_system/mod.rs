use crate::wgl;
use pest::error::Error;

use rand::prelude::*;
use std::str::FromStr;

mod compiler;
mod distribution;
mod types;

pub mod phone;
pub mod rules;
pub use compiler::*;
pub use types::*;

impl MonoSyllableRepartition {
    pub fn into_percentage(self) -> f32 {
        match self {
            MonoSyllableRepartition::Always => 1.0,
            MonoSyllableRepartition::Mostly => 0.85,
            MonoSyllableRepartition::Frequent => 0.5,
            MonoSyllableRepartition::LessFrequent => 0.20,
            MonoSyllableRepartition::Rare => 0.07,
            MonoSyllableRepartition::Never => 0.0,
        }
    }
}

impl FromStr for MonoSyllableRepartition {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(MonoSyllableRepartition::Always),
            "mostly" => Ok(MonoSyllableRepartition::Mostly),
            "frequent" => Ok(MonoSyllableRepartition::Frequent),
            "less_frequent" => Ok(MonoSyllableRepartition::LessFrequent),
            "rare" => Ok(MonoSyllableRepartition::Rare),
            "never" => Ok(MonoSyllableRepartition::Never),
            _ => Err("no match"),
        }
    }
}

pub fn from_string(input: &'_ str) -> Result<SoundSystem, Error<wgl::Rule>> {
    wgl::from_string(input).map(SoundSystem::compile)
}

impl SoundSystem {
    fn add_phonemes(&mut self, repr: &'_ str, phones: phone::Phones) {
        self.phonemes()
            .insert(repr.to_string(), (phones, Condition::Always));
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

    pub fn generate_words(
        &self,
        number: usize,
        repartition: MonoSyllableRepartition,
    ) -> Vec<String> {
        let mut words = vec![];
        let percentage = repartition.into_percentage();
        for _ in 0..number {
            let mut number_of_syllables = 1;
            if percentage < 1.0 && random::<f32>() > percentage {
                number_of_syllables += 1 + distribution::power_law(4, 0.5);
            }
            let mut word = String::new();
            for _ in 0..number_of_syllables {
                word.push_str(&self.syllable());
            }
            if !words.contains(&word) {
                words.push(word);
            }
        }
        words
    }

    fn syllable(&self) -> String {
        let syllables_size = self.syllables().len();
        let syllable_drop = syllable_drop(syllables_size);
        let mut syllable = String::new();
        let index = distribution::power_law(syllables_size, syllable_drop);
        let pattern = &self.syllables()[index];

        for class_name in pattern {
            if let Some(letters) = self.classes().get(class_name) {
                let distribution = self
                    .distribution()
                    .iter()
                    .filter(|t| letters.contains(&t.0))
                    .collect::<Vec<_>>();
                let letter = distribution::select(distribution);
                syllable.push_str(letter);
            }
        }
        syllable
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
        letters_condition: &'a (String, PhonemeCondition),
        position: usize,
        length: usize,
    ) -> Option<(&'a String, &'a phone::Phones)> {
        let (letter, (phones, condition)) = letters_condition;
        let result = (letter, phones);
        if input.starts_with(letter)
            && self.resolve_condition(input, letter, position, length, condition)
        {
            Some(result)
        } else {
            None
        }
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
            Condition::Binary(op, left, right) => {
                let left_bool = self.resolve_condition(input, letter, position, length, left);
                let right_bool = self.resolve_condition(input, letter, position, length, right);
                match op {
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

fn syllable_drop(number_of_syllables: usize) -> f32 {
    if number_of_syllables < 9 {
        0.6 - (number_of_syllables as f32) * 0.05
    } else {
        0.12
    }
}

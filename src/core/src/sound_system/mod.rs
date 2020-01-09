use crate::wgl;
use pest::error::Error;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use rand::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

mod compiler;
mod distribution;
pub mod phone;
pub mod rules;
pub use compiler::*;
use std::convert::Into;

#[derive(Default, Debug, Serialize)]
pub struct SoundSystem {
    classes: HashMap<String, Vec<String>>,
    phonemes: HashMap<String, phone::Phones>,
    syllables: Vec<Vec<String>>,
    distribution: Vec<(String, f64)>,
    rules: Vec<Rule>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug)]
pub enum MonoSyllableRepartition {
    Always,
    Mostly,
    Frequent,
    LessFrequent,
    Rare,
    Never,
}

#[cfg(not(feature = "wasm"))]
#[repr(u8)]
#[derive(Debug)]
pub enum MonoSyllableRepartition {
    Always,
    Mostly,
    Frequent,
    LessFrequent,
    Rare,
    Never,
}
#[derive(Debug, Serialize, Clone)]
pub enum Rule {
    SoundRule(SoundRule),
    PhonemeRule(PhonemeRule),
}

#[derive(Debug, Serialize, Clone)]
pub struct SoundRule {
    name: String,
    regex: String,
    replacement: Option<String>,
}
#[derive(Debug, Serialize, Default, Clone)]
pub struct PhonemeRule {
    name: String,
    phoneme_differences: Vec<PhonemeDifference>,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum PhonemeDifference {
    Skip,
    Delete(String),
    Upsert(String, phone::Phones),
}

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
    fn new() -> Self {
        Default::default()
    }

    fn add_phonemes(&mut self, repr: &'_ str, phones: phone::Phones) {
        self.phonemes.insert(repr.to_string(), phones);
    }
    pub fn update_phoneme(&mut self, diffs: &[PhonemeDifference]) {
        diffs.iter().for_each(|diff| match diff {
            PhonemeDifference::Skip => (),
            PhonemeDifference::Delete(repr) => {
                self.phonemes.remove(repr);
            }
            PhonemeDifference::Upsert(repr, phones) => self.add_phonemes(repr, phones.clone()),
        })
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
        let syllables_size = self.syllables.len();
        let syllable_drop = syllable_drop(syllables_size);
        let mut syllable = String::new();
        let index = distribution::power_law(syllables_size, syllable_drop);
        let pattern = &self.syllables[index];

        for class_name in pattern {
            if let Some(letters) = self.classes.get(class_name) {
                let distribution = self
                    .distribution
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
        let mut phonemes = self.phonemes.iter().collect::<Vec<_>>();
        phonemes.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));
        let mut input = word.to_string();
        while !input.is_empty() {
            match phonemes
                .iter()
                .find(|(letter, _)| input.starts_with(*letter))
            {
                Some((letter, phones)) => {
                    phones
                        .iter()
                        .map(|p| p.clone().into())
                        .for_each(|c| result.push(c));
                    input = input[letter.len()..].to_string();
                }
                None => {
                    input = input[1..].to_string();
                }
            }
        }
        result
    }
}

fn syllable_drop(number_of_syllables: usize) -> f32 {
    if number_of_syllables < 9 {
        0.6 - (number_of_syllables as f32) * 0.05
    } else {
        0.12
    }
}

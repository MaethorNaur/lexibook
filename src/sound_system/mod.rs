use rand::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

mod compiler;
mod distribution;
pub mod phone;
pub use compiler::*;

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

#[derive(Debug)]
pub struct SoundSystem<'a> {
    classes: HashMap<&'a str, Vec<&'a str>>,
    phonemes: HashMap<&'a str, phone::Phones>,
    syllables: Vec<Vec<&'a str>>,
    distribution: Vec<(&'a str, f64)>,
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

impl<'a> SoundSystem<'a> {
    fn new() -> Self {
        Self {
            classes: HashMap::new(),
            phonemes: HashMap::new(),
            syllables: vec![],
            distribution: vec![],
        }
    }

    fn add_phonemes(&mut self, repr: &'a str, phones: phone::Phones) {
        self.phonemes.insert(repr, phones);
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
}

fn syllable_drop(number_of_syllables: usize) -> f32 {
    if number_of_syllables < 9 {
        0.6 - (number_of_syllables as f32) * 0.05
    } else {
        0.12
    }
}

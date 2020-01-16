use crate::sound_system::phone;
use crate::wgl;
use std::collections::HashMap;
use std::convert::Into;
pub type Syllable = Vec<String>;
pub type Distribution = (String, f64);
pub type PhonemeCondition = (phone::Phones, Condition);
pub type Phoneme = (String, PhonemeCondition);

#[derive(Default, Debug, Serialize)]
pub struct SoundSystem {
    classes: HashMap<String, Vec<String>>,
    phonemes: HashMap<String, PhonemeCondition>,
    phonemes_sorted: Vec<Phoneme>,
    syllables: Vec<Syllable>,
    distribution: Vec<Distribution>,
    rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Clone)]
pub enum Rule {
    SoundRule {
        name: String,
        regex: String,
        replacement: Option<String>,
    },
    PhonemeRule {
        name: String,
        phoneme_differences: Vec<PhonemeDifference>,
    },
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum PhonemeDifference {
    Skip,
    Delete(String),
    Upsert(String, phone::Phones),
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum Condition {
    Always,
    Single(ConditionType),
    Not(ConditionType),
    Binary {
        operand: ConditionOperand,
        left: Box<Condition>,
        right: Box<Condition>,
    },
}

impl<'a> Into<Condition> for wgl::Condition<'a> {
    fn into(self) -> Condition {
        match self {
            wgl::Condition::Always => Condition::Always,
            wgl::Condition::Single(single) => Condition::Single(single.into()),
            wgl::Condition::Not(not) => Condition::Not(not.into()),
            wgl::Condition::And(left, right) => Condition::Binary {
                operand: ConditionOperand::And,
                left: Box::new((*left).into()),
                right: Box::new((*right).into()),
            },
            wgl::Condition::Or(left, right) => Condition::Binary {
                operand: ConditionOperand::Or,
                left: Box::new((*left).into()),
                right: Box::new((*right).into()),
            },
        }
    }
}
#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum ConditionOperand {
    And,
    Or,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum ConditionType {
    None,
    BeginningWord,
    EndWord,
    FollowedBy(String),
    Between(String, String),
}

impl<'a> Into<ConditionType> for wgl::ConditionType<'a> {
    fn into(self) -> ConditionType {
        match self {
            wgl::ConditionType::None => ConditionType::None,
            wgl::ConditionType::BeginningWord => ConditionType::BeginningWord,
            wgl::ConditionType::EndWord => ConditionType::EndWord,
            wgl::ConditionType::FollowedBy(c) => ConditionType::FollowedBy(c.to_string()),
            wgl::ConditionType::Between(l, r) => {
                ConditionType::Between(l.to_string(), r.to_string())
            }
        }
    }
}

impl SoundSystem {
    pub fn with_default() -> Self {
        Default::default()
    }

    pub fn new(
        classes: HashMap<String, Vec<String>>,
        phonemes: HashMap<String, PhonemeCondition>,
        syllables: Vec<Syllable>,
        distribution: Vec<(String, f64)>,
        rules: Vec<Rule>,
    ) -> Self {
        let phonemes_sorted = sort_phonemes(&phonemes);
        Self {
            classes,
            phonemes,
            syllables,
            distribution,
            rules,
            phonemes_sorted,
        }
    }

    pub fn sort_phonemes(&mut self) {
        self.phonemes_sorted = sort_phonemes(&self.phonemes)
    }

    pub fn phonemes_sorted(&self) -> &Vec<Phoneme> {
        &self.phonemes_sorted
    }

    pub fn phonemes(&mut self) -> &mut HashMap<String, PhonemeCondition> {
        &mut self.phonemes
    }

    pub fn syllables(&self) -> &Vec<Syllable> {
        &self.syllables
    }

    pub fn classes(&self) -> &HashMap<String, Vec<String>> {
        &self.classes
    }

    pub fn distribution(&self) -> &Vec<(String, f64)> {
        &self.distribution
    }

    pub fn rules(&self) -> &Vec<Rule> {
        &self.rules
    }
}

fn sort_phonemes(current: &HashMap<String, PhonemeCondition>) -> Vec<Phoneme> {
    let mut phonemes = current.clone().into_iter().collect::<Vec<_>>();
    phonemes.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));
    phonemes
}

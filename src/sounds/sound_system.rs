use crate::sounds::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SoundSystem<'a> {
    pub letters: Vec<Letter<'a>>,
    pub classes: HashMap<String, Vec<usize>>,
    pub syllables: Vec<Vec<Syllable<'a>>>,
}
pub type Letter<'a> = (&'a str, Vec<IPASound>);

impl<'a> SoundSystem<'a> {
    pub fn new() -> Self {
        SoundSystem {
            letters: Vec::new(),
            classes: HashMap::new(),
            syllables: Vec::new(),
        }
    }
    pub fn add(&mut self, letter: Letter<'a>) {
        self.letters.push(letter);
    }
    pub fn add_classes<'b>(
        &mut self,
        class: &'b str,
        letters: Vec<&'b str>,
    ) -> Result<(), &'b str> {
        let mut letters_indices: Vec<usize> = Vec::new();
        let mut letters_iter = self.letters.iter();
        for letter in letters.iter() {
            if let Some(index) = letters_iter.position(|(l, _sounds)| l == letter) {
                letters_indices.push(index);
            } else {
                return Err(letter);
            }
        }
        self.classes.insert(class.to_string(), letters_indices);
        Ok(())
    }
    pub fn generate_default_classes(&mut self) {
        let consonents: Vec<usize> = self
            .letters
            .iter()
            .enumerate()
            .filter(|(_index, (_letter, sounds))| {
                sounds
                    .iter()
                    .all(|sound| sound.type_of_sound == Type::Consonant)
            })
            .map(|(index, _letter)| index)
            .collect();
        let vowels: Vec<usize> = self
            .letters
            .iter()
            .enumerate()
            .filter(|(_index, (_letter, sounds))| {
                sounds
                    .iter()
                    .any(|sound| sound.type_of_sound == Type::Vowel)
            })
            .map(|(index, _letter)| index)
            .collect();
        self.classes.insert("C".to_string(), consonents);
        self.classes.insert("V".to_string(), vowels);
    }
}
pub fn new_letter<'a>(value: &'a str, string_sound: &'a str) -> Letter<'a> {
    let sounds: Vec<IPASound> = string_sound.chars().map(IPASound::from).collect();
    (value, sounds)
}

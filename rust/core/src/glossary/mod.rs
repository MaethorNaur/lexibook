use crate::sound_system::SoundSystem;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Glossary {
    sound_system: SoundSystem,
}

impl Glossary {
    pub fn new(sound_system: SoundSystem) -> Self {
        Self { sound_system }
    }

    pub fn open(path: &'_ str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let input = brotli::Decompressor::new(file, 4096);
        let glossary = bincode::deserialize_from(input)?;
        Ok(glossary)
    }

    pub fn save(&self, path: &'_ str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let compressor = brotli::CompressorWriter::new(file, 4096, 11, 22);
        bincode::serialize_into(compressor, &self)?;
        Ok(())
    }

    pub fn sound_system(&self) -> &SoundSystem {
        &self.sound_system
    }
}

use lexibook::sound_system;
use log::LevelFilter;
use std::convert::Into;
mod lists;
pub use lists::*;

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

impl Into<sound_system::MonoSyllableRepartition> for MonoSyllableRepartition {
    fn into(self) -> sound_system::MonoSyllableRepartition {
        match self {
            MonoSyllableRepartition::Always => sound_system::MonoSyllableRepartition::Always,
            MonoSyllableRepartition::Mostly => sound_system::MonoSyllableRepartition::Mostly,
            MonoSyllableRepartition::Frequent => sound_system::MonoSyllableRepartition::Frequent,
            MonoSyllableRepartition::LessFrequent => {
                sound_system::MonoSyllableRepartition::LessFrequent
            }
            MonoSyllableRepartition::Rare => sound_system::MonoSyllableRepartition::Rare,
            MonoSyllableRepartition::Never => sound_system::MonoSyllableRepartition::Never,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn to_level(self) -> LevelFilter {
        match self {
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
        }
    }
}

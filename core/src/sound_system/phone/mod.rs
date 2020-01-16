use std::convert::{Into, TryFrom};
mod classes;
mod ordering;
mod types;
use std::collections::HashMap;
use std::fmt;
pub use types::*;
lazy_static! {
    static ref PHONES: HashMap<char, Phone> = {
        let mut map = HashMap::new();
        map.insert(
            'p',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'b',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            't',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'd',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʈ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ɖ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'c',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ɟ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'k',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'g',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'q',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ɢ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʡ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Pharyngeal,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ʔ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Glottal,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'm',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɱ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'n',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɳ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɲ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ŋ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɴ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʙ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Trill,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'r',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Trill,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʀ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Trill,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ⱱ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Tap,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɾ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Tap,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɽ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Tap,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɸ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'β',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'f',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'v',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'θ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Dental,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ð',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Dental,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            's',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::SibilantFricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'z',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::SibilantFricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʃ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::PostAlveolar,
                manner: ConsonantManner::SibilantFricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ʒ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::PostAlveolar,
                manner: ConsonantManner::SibilantFricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʂ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::SibilantFricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ʐ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::SibilantFricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɕ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::SibilantFricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ʑ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::SibilantFricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ç',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ʝ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'x',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ɣ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'χ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ʁ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ħ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Pharyngeal,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ʕ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Pharyngeal,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'h',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Glottal,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ɦ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Glottal,
                manner: ConsonantManner::Fricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɬ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralFricative,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ɮ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralFricative,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʋ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɹ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɻ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'j',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'w',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioVelar,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɰ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'l',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralApproximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ɭ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::LateralApproximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʎ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::LateralApproximant,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'ʟ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::LateralApproximant,
                phonation: Phonation::Voiceless,
            }),
        );
        map.insert(
            'ɺ',
            Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralTap,
                phonation: Phonation::Voiced,
            }),
        );
        map.insert(
            'i',
            Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'y',
            Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ɨ',
            Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Central,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ʉ',
            Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Central,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ɯ',
            Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Back,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'u',
            Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ɪ',
            Phone::Vowel(Vowel {
                height: Height::NearClose,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ʏ',
            Phone::Vowel(Vowel {
                height: Height::NearClose,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ʊ',
            Phone::Vowel(Vowel {
                height: Height::NearClose,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'e',
            Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ø',
            Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ɘ',
            Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Central,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ɵ',
            Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Central,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ɤ',
            Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Back,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'o',
            Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ə',
            Phone::Vowel(Vowel {
                height: Height::Mid,
                backness: Backness::Central,
                roundness: Roundness::Undefined,
            }),
        );
        map.insert(
            'ɛ',
            Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'œ',
            Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ɜ',
            Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Central,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ɞ',
            Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Central,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ʌ',
            Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Back,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ɔ',
            Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'æ',
            Phone::Vowel(Vowel {
                height: Height::NearOpen,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ɐ',
            Phone::Vowel(Vowel {
                height: Height::NearOpen,
                backness: Backness::Central,
                roundness: Roundness::Undefined,
            }),
        );
        map.insert(
            'a',
            Phone::Vowel(Vowel {
                height: Height::Open,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ɶ',
            Phone::Vowel(Vowel {
                height: Height::Open,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ɑ',
            Phone::Vowel(Vowel {
                height: Height::Open,
                backness: Backness::Back,
                roundness: Roundness::UnRounded,
            }),
        );
        map.insert(
            'ɒ',
            Phone::Vowel(Vowel {
                height: Height::Open,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
            }),
        );
        map.insert(
            'ʰ',
            Phone::Diacritic(PhoneProperty::ConsonantRelease(ConsonantRelease::Aspirated)),
        );
        map.insert(
            '\u{31A}',
            Phone::Diacritic(PhoneProperty::ConsonantRelease(ConsonantRelease::NoAudible)),
        );
        map.insert(
            'ⁿ',
            Phone::Diacritic(PhoneProperty::ConsonantRelease(ConsonantRelease::Nasal)),
        );
        map.insert(
            'ˡ',
            Phone::Diacritic(PhoneProperty::ConsonantRelease(ConsonantRelease::Lateral)),
        );
        map.insert(
            'ᶿ',
            Phone::Diacritic(PhoneProperty::ConsonantRelease(ConsonantRelease::Dental)),
        );
        map.insert(
            'ˣ',
            Phone::Diacritic(PhoneProperty::ConsonantRelease(ConsonantRelease::Velar)),
        );
        map.insert(
            'ᵊ',
            Phone::Diacritic(PhoneProperty::ConsonantRelease(
                ConsonantRelease::MidCentralVowel,
            )),
        );
        map.insert(
            '\u{325}',
            Phone::Diacritic(PhoneProperty::Phonation(Phonation::Voiceless)),
        );
        map.insert(
            '\u{30A}',
            Phone::Diacritic(PhoneProperty::Phonation(Phonation::Voiceless)),
        );
        map.insert(
            '\u{32C}',
            Phone::Diacritic(PhoneProperty::Phonation(Phonation::Voiced)),
        );
        map.insert(
            '\u{324}',
            Phone::Diacritic(PhoneProperty::Phonation(Phonation::BreathyVoiced)),
        );
        map.insert(
            'ʱ',
            Phone::Diacritic(PhoneProperty::Phonation(Phonation::BreathyVoiced)),
        );
        map.insert(
            '\u{330}',
            Phone::Diacritic(PhoneProperty::Phonation(Phonation::CreakyVoiced)),
        );
        map.insert(
            '\u{32A}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Dental)),
        );
        map.insert(
            '\u{33C}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Linguolabial)),
        );
        map.insert(
            '\u{33A}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Apical)),
        );
        map.insert(
            '\u{33B}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Laminal)),
        );
        map.insert(
            '\u{2D6}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Advanced)),
        );
        map.insert(
            '\u{31F}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Advanced)),
        );
        map.insert(
            '\u{2D7}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Retracted)),
        );
        map.insert(
            '\u{320}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Retracted)),
        );
        map.insert(
            '\u{308}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Centralized)),
        );
        map.insert(
            '\u{33D}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::MidCentralized)),
        );
        map.insert(
            '\u{2D4}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Raised)),
        );
        map.insert(
            '\u{31D}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Raised)),
        );
        map.insert(
            '\u{2D5}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Lowered)),
        );
        map.insert(
            '\u{31E}',
            Phone::Diacritic(PhoneProperty::Articulation(Articulation::Lowered)),
        );
        map.insert(
            '\u{339}',
            Phone::Diacritic(PhoneProperty::CoArticulation(CoArticulation::MoreRounded)),
        );
        map.insert(
            '\u{31C}',
            Phone::Diacritic(PhoneProperty::CoArticulation(CoArticulation::LessRounded)),
        );
        map.insert(
            'ʷ',
            Phone::Diacritic(PhoneProperty::CoArticulation(CoArticulation::Labialized)),
        );
        map.insert(
            'ʲ',
            Phone::Diacritic(PhoneProperty::CoArticulation(CoArticulation::Palatalized)),
        );
        map.insert(
            'ˤ',
            Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::Pharyngealized,
            )),
        );
        map.insert(
            'ˠ',
            Phone::Diacritic(PhoneProperty::CoArticulation(CoArticulation::Velarized)),
        );
        map.insert(
            '\u{334}',
            Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::VelarizedOrPharyngealized,
            )),
        );
        map.insert(
            '\u{318}',
            Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::AdvancedTongueRoot,
            )),
        );
        map.insert(
            '\u{319}',
            Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::RetractedTongueRoot,
            )),
        );
        map.insert(
            '\u{303}',
            Phone::Diacritic(PhoneProperty::CoArticulation(CoArticulation::Nasalized)),
        );
        map.insert(
            '\u{2DE}',
            Phone::Diacritic(PhoneProperty::CoArticulation(CoArticulation::Rhoticity)),
        );
        map.insert(
            'ɚ',
            Phone::Vowel(Vowel {
                height: Height::Mid,
                backness: Backness::Central,
                roundness: Roundness::Undefined,
            }),
        );
        map.insert(
            'ˈ',
            Phone::Diacritic(PhoneProperty::Rhythm(Rhythm::PrimaryStress)),
        );
        map.insert(
            'ˌ',
            Phone::Diacritic(PhoneProperty::Rhythm(Rhythm::SecondaryStress)),
        );
        map.insert('ː', Phone::Diacritic(PhoneProperty::Rhythm(Rhythm::Long)));
        map.insert(
            'ˑ',
            Phone::Diacritic(PhoneProperty::Rhythm(Rhythm::HalfLong)),
        );
        map.insert(
            '\u{306}',
            Phone::Diacritic(PhoneProperty::Rhythm(Rhythm::ExtraShort)),
        );
        map.insert('\u{30B}', Phone::Diacritic(PhoneProperty::Tone(Tone::Top)));
        map.insert('˥', Phone::Diacritic(PhoneProperty::Tone(Tone::Top)));
        map.insert('˦', Phone::Diacritic(PhoneProperty::Tone(Tone::High)));
        map.insert('\u{301}', Phone::Diacritic(PhoneProperty::Tone(Tone::High)));
        map.insert('\u{304}', Phone::Diacritic(PhoneProperty::Tone(Tone::Mid)));
        map.insert('˧', Phone::Diacritic(PhoneProperty::Tone(Tone::Mid)));
        map.insert('\u{300}', Phone::Diacritic(PhoneProperty::Tone(Tone::Low)));
        map.insert('˨', Phone::Diacritic(PhoneProperty::Tone(Tone::Low)));
        map.insert(
            '\u{30F}',
            Phone::Diacritic(PhoneProperty::Tone(Tone::Bottom)),
        );
        map.insert('˩', Phone::Diacritic(PhoneProperty::Tone(Tone::Bottom)));
        map.insert(
            '\u{30C}',
            Phone::Diacritic(PhoneProperty::Tone(Tone::Rising)),
        );
        map.insert(
            '\u{302}',
            Phone::Diacritic(PhoneProperty::Tone(Tone::Falling)),
        );
        map.insert('ꜛ', Phone::Diacritic(PhoneProperty::Tone(Tone::Upstep)));
        map.insert('ꜜ', Phone::Diacritic(PhoneProperty::Tone(Tone::Downstep)));
        map.insert(
            '|',
            Phone::Diacritic(PhoneProperty::Intonation(Intonation::MinorBreak)),
        );
        map.insert(
            '‖',
            Phone::Diacritic(PhoneProperty::Intonation(Intonation::MajorBreak)),
        );
        map.insert(
            '↗',
            Phone::Diacritic(PhoneProperty::Intonation(Intonation::GlobalRise)),
        );
        map.insert(
            '↘',
            Phone::Diacritic(PhoneProperty::Intonation(Intonation::GlobalFall)),
        );
        map
    };
}

impl Into<char> for Phone {
    fn into(self) -> char {
        PHONES
            .iter()
            .find(|(_, phone)| **phone == self)
            .map(|t| *t.0)
            .unwrap()
    }
}

impl TryFrom<&'_ str> for Phone {
    type Error = &'static str;
    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        value
            .chars()
            .next()
            .ok_or("Empty string")
            .and_then(Phone::try_from)
    }
}

impl TryFrom<char> for Phone {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        PHONES.get(&value).copied().ok_or("no match")
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = PHONES
            .iter()
            .find(|(_, phone)| *phone == self)
            .map(|t| *t.0)
            .unwrap();
        write!(f, "{}", c)
    }
}

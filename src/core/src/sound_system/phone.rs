use std::convert::{Into, TryFrom};
pub type Phones = Vec<Phone>;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Phone {
    Consonant(Consonant),
    Vowel(Vowel),
    Diacritic(PhoneProperty),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct Vowel {
    pub height: Height,
    pub backness: Backness,
    pub roundness: Roundness,
    pub properties: Vec<PhoneProperty>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Height {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Backness {
    Front,
    Central,
    Back,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Roundness {
    Rounded,
    UnRounded,
    Undefined,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct Consonant {
    pub place: ConsonantPlace,
    pub manner: ConsonantManner,
    pub properties: Vec<PhoneProperty>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum ConsonantPlace {
    Bilabial,
    LabioDental,
    LabioVelar,
    Dental,
    Alveolar,
    PostAlveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glotal,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum ConsonantManner {
    Stop,
    Nasal,
    Trill,
    Tap,
    Fricative,
    SibilantFricative,
    LateralFricative,
    Approximant,
    LateralApproximant,
    LateralTap,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum PhoneProperty {
    ConsonantRelease(ConsonantRelease),
    Phonation(Phonation),
    Articulation(Articulation),
    CoArticulation(CoArticulation),
    Rhythm(Rhythm),
    Tone(Tone),
    Intonation(Intonation),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum ConsonantRelease {
    Aspirated,
    NoAudible,
    Nasal,
    Lateral,
    Dental,
    Velar,
    MidCentralVowel,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Phonation {
    Voiced,
    Voiceless,
    BreathyVoiced,
    CreakyVoiced,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Articulation {
    Dental,
    Linguolabial,
    Apical,
    Laminal,
    Advanced,
    Retracted,
    Raised,
    Lowered,
    Centralized,
    MidCentralized,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum CoArticulation {
    MoreRounded,
    LessRounded,
    Labialized,
    Palatalized,
    Velarized,
    Pharyngealized,
    VelarizedOrPharyngealized,
    AdvancedTongueRoot,
    RetractedTongueRoot,
    Nasalized,
    Rhoticity,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Rhythm {
    PrimaryStress,
    SecondaryStress,
    Long,
    HalfLong,
    ExtraShort,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Intonation {
    MinorBreak,
    MajorBreak,
    GlobalRise,
    GlobalFall,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Tone {
    Top,
    Upstep,
    High,
    Rising,
    Mid,
    Low,
    Falling,
    Bottom,
    Downstep,
}

impl Into<char> for Phone {
    fn into(self) -> char {
        let res = match self {
            Phone::Consonant(consonant) => consonant_ipa(consonant),
            Phone::Vowel(vowel) => vowel_ipa(vowel),
            Phone::Diacritic(property) => diacritic_ipa(property),
        };
        res.unwrap()
    }
}

fn diacritic_ipa(property: PhoneProperty) -> Option<char> {
    match property {
        PhoneProperty::ConsonantRelease(ConsonantRelease::Aspirated) => Some('ʰ'),
        PhoneProperty::ConsonantRelease(ConsonantRelease::NoAudible) => Some('\u{31A}'),
        PhoneProperty::ConsonantRelease(ConsonantRelease::Nasal) => Some('ⁿ'),
        PhoneProperty::ConsonantRelease(ConsonantRelease::Lateral) => Some('ˡ'),
        PhoneProperty::ConsonantRelease(ConsonantRelease::Dental) => Some('ᶿ'),
        PhoneProperty::ConsonantRelease(ConsonantRelease::Velar) => Some('ˣ'),
        PhoneProperty::ConsonantRelease(ConsonantRelease::MidCentralVowel) => Some('ᵊ'),
        PhoneProperty::Phonation(Phonation::Voiceless) => Some('\u{325}'),
        PhoneProperty::Phonation(Phonation::Voiced) => Some('\u{32C}'),
        PhoneProperty::Phonation(Phonation::BreathyVoiced) => Some('ʱ'),
        PhoneProperty::Phonation(Phonation::CreakyVoiced) => Some('\u{330}'),
        PhoneProperty::Articulation(Articulation::Dental) => Some('\u{32A}'),
        PhoneProperty::Articulation(Articulation::Linguolabial) => Some('\u{33C}'),
        PhoneProperty::Articulation(Articulation::Apical) => Some('\u{33A}'),
        PhoneProperty::Articulation(Articulation::Laminal) => Some('\u{33B}'),
        PhoneProperty::Articulation(Articulation::Advanced) => Some('\u{31F}'),
        PhoneProperty::Articulation(Articulation::Retracted) => Some('\u{320}'),
        PhoneProperty::Articulation(Articulation::Centralized) => Some('\u{308}'),
        PhoneProperty::Articulation(Articulation::MidCentralized) => Some('\u{33D}'),
        PhoneProperty::Articulation(Articulation::Raised) => Some('\u{31D}'),
        PhoneProperty::Articulation(Articulation::Lowered) => Some('\u{31E}'),
        PhoneProperty::CoArticulation(CoArticulation::MoreRounded) => Some('\u{339}'),
        PhoneProperty::CoArticulation(CoArticulation::LessRounded) => Some('\u{31C}'),
        PhoneProperty::CoArticulation(CoArticulation::Labialized) => Some('ʷ'),
        PhoneProperty::CoArticulation(CoArticulation::Palatalized) => Some('ʲ'),
        PhoneProperty::CoArticulation(CoArticulation::Pharyngealized) => Some('ˤ'),
        PhoneProperty::CoArticulation(CoArticulation::Velarized) => Some('ˠ'),
        PhoneProperty::CoArticulation(CoArticulation::VelarizedOrPharyngealized) => Some('\u{334}'),
        PhoneProperty::CoArticulation(CoArticulation::AdvancedTongueRoot) => Some('\u{318}'),
        PhoneProperty::CoArticulation(CoArticulation::RetractedTongueRoot) => Some('\u{319}'),
        PhoneProperty::CoArticulation(CoArticulation::Nasalized) => Some('\u{303}'),
        PhoneProperty::CoArticulation(CoArticulation::Rhoticity) => Some('\u{2DE}'),
        PhoneProperty::Rhythm(Rhythm::PrimaryStress) => Some('ˈ'),
        PhoneProperty::Rhythm(Rhythm::SecondaryStress) => Some('ˌ'),
        PhoneProperty::Rhythm(Rhythm::Long) => Some('ː'),
        PhoneProperty::Rhythm(Rhythm::HalfLong) => Some('ˑ'),
        PhoneProperty::Rhythm(Rhythm::ExtraShort) => Some('\u{306}'),
        PhoneProperty::Tone(Tone::Top) => Some('\u{30B}'),
        PhoneProperty::Tone(Tone::High) => Some('\u{301}'),
        PhoneProperty::Tone(Tone::Mid) => Some('\u{304}'),
        PhoneProperty::Tone(Tone::Low) => Some('\u{300}'),
        PhoneProperty::Tone(Tone::Bottom) => Some('\u{30F}'),
        PhoneProperty::Tone(Tone::Rising) => Some('\u{30C}'),
        PhoneProperty::Tone(Tone::Falling) => Some('\u{302}'),
        PhoneProperty::Tone(Tone::Upstep) => Some('ꜛ'),
        PhoneProperty::Tone(Tone::Downstep) => Some('ꜜ'),
        PhoneProperty::Intonation(Intonation::MinorBreak) => Some('|'),
        PhoneProperty::Intonation(Intonation::MajorBreak) => Some('‖'),
        PhoneProperty::Intonation(Intonation::GlobalRise) => Some('↗'),
        PhoneProperty::Intonation(Intonation::GlobalFall) => Some('↘'),
    }
}

fn vowel_ipa(vowel: Vowel) -> Option<char> {
    let to_match = (
        vowel.height,
        vowel.backness,
        vowel.roundness,
        vowel.properties.as_slice(),
    );
    match to_match {
        (Height::Close, Backness::Front, Roundness::UnRounded, []) => Some('i'),
        (Height::Close, Backness::Front, Roundness::Rounded, []) => Some('y'),
        (Height::Close, Backness::Central, Roundness::UnRounded, []) => Some('ɨ'),
        (Height::Close, Backness::Central, Roundness::Rounded, []) => Some('ʉ'),
        (Height::Close, Backness::Back, Roundness::UnRounded, []) => Some('ɯ'),
        (Height::Close, Backness::Back, Roundness::Rounded, []) => Some('u'),
        (Height::NearClose, Backness::Front, Roundness::UnRounded, []) => Some('ɪ'),
        (Height::NearClose, Backness::Front, Roundness::Rounded, []) => Some('ʏ'),
        (Height::NearClose, Backness::Back, Roundness::Rounded, []) => Some('ʊ'),
        (Height::CloseMid, Backness::Front, Roundness::UnRounded, []) => Some('e'),
        (Height::CloseMid, Backness::Front, Roundness::Rounded, []) => Some('ø'),
        (Height::CloseMid, Backness::Central, Roundness::UnRounded, []) => Some('ɘ'),
        (Height::CloseMid, Backness::Central, Roundness::Rounded, []) => Some('ɵ'),
        (Height::CloseMid, Backness::Back, Roundness::UnRounded, []) => Some('ɤ'),
        (Height::CloseMid, Backness::Back, Roundness::Rounded, []) => Some('o'),
        (Height::Mid, Backness::Central, Roundness::Undefined, []) => Some('ə'),
        (Height::OpenMid, Backness::Front, Roundness::UnRounded, []) => Some('ɛ'),
        (Height::OpenMid, Backness::Front, Roundness::Rounded, []) => Some('œ'),
        (Height::OpenMid, Backness::Central, Roundness::UnRounded, []) => Some('ɜ'),
        (Height::OpenMid, Backness::Central, Roundness::Rounded, []) => Some('ɞ'),
        (Height::OpenMid, Backness::Back, Roundness::UnRounded, []) => Some('ʌ'),
        (Height::OpenMid, Backness::Back, Roundness::Rounded, []) => Some('ɔ'),
        (Height::NearOpen, Backness::Front, Roundness::UnRounded, []) => Some('æ'),
        (Height::NearOpen, Backness::Central, Roundness::Undefined, []) => Some('ɐ'),
        (Height::Open, Backness::Front, Roundness::UnRounded, []) => Some('a'),
        (Height::Open, Backness::Front, Roundness::Rounded, []) => Some('ɶ'),
        (Height::Open, Backness::Back, Roundness::UnRounded, []) => Some('ɑ'),
        (Height::Open, Backness::Back, Roundness::Rounded, []) => Some('ɒ'),
        (
            Height::Mid,
            Backness::Central,
            Roundness::Undefined,
            [PhoneProperty::CoArticulation(CoArticulation::Rhoticity)],
        ) => Some('ɚ'),

        _ => None,
    }
}

fn consonant_ipa(consonant: Consonant) -> Option<char> {
    let to_match = (
        consonant.place,
        consonant.manner,
        consonant.properties.as_slice(),
    );
    match to_match {
        (
            ConsonantPlace::Bilabial,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('p'),
        (
            ConsonantPlace::Bilabial,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('b'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('t'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('d'),
        (
            ConsonantPlace::Retroflex,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ʈ'),
        (
            ConsonantPlace::Retroflex,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɖ'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('c'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɟ'),
        (
            ConsonantPlace::Velar,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('k'),
        (
            ConsonantPlace::Velar,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('g'),
        (
            ConsonantPlace::Uvular,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('q'),
        (
            ConsonantPlace::Uvular,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɢ'),
        (
            ConsonantPlace::Pharyngeal,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ʡ'),
        (
            ConsonantPlace::Glotal,
            ConsonantManner::Stop,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ʔ'),
        (
            ConsonantPlace::Bilabial,
            ConsonantManner::Nasal,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('m'),
        (
            ConsonantPlace::LabioDental,
            ConsonantManner::Nasal,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɱ'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::Nasal,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('n'),
        (
            ConsonantPlace::Retroflex,
            ConsonantManner::Nasal,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɳ'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::Nasal,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɲ'),
        (
            ConsonantPlace::Velar,
            ConsonantManner::Nasal,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ŋ'),
        (
            ConsonantPlace::Uvular,
            ConsonantManner::Nasal,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɴ'),
        (
            ConsonantPlace::Bilabial,
            ConsonantManner::Trill,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʙ'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::Trill,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('r'),
        (
            ConsonantPlace::Uvular,
            ConsonantManner::Trill,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʀ'),
        (
            ConsonantPlace::LabioDental,
            ConsonantManner::Tap,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ⱱ'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::Tap,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɾ'),
        (
            ConsonantPlace::Retroflex,
            ConsonantManner::Tap,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɽ'),
        (
            ConsonantPlace::Bilabial,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ɸ'),
        (
            ConsonantPlace::Bilabial,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('β'),
        (
            ConsonantPlace::LabioDental,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('f'),
        (
            ConsonantPlace::LabioDental,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('v'),
        (
            ConsonantPlace::Dental,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('θ'),
        (
            ConsonantPlace::Dental,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ð'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::SibilantFricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('s'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::SibilantFricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('z'),
        (
            ConsonantPlace::PostAlveolar,
            ConsonantManner::SibilantFricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ʃ'),
        (
            ConsonantPlace::PostAlveolar,
            ConsonantManner::SibilantFricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʒ'),
        (
            ConsonantPlace::Retroflex,
            ConsonantManner::SibilantFricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ʂ'),
        (
            ConsonantPlace::Retroflex,
            ConsonantManner::SibilantFricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʐ'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::SibilantFricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ɕ'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::SibilantFricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʑ'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ç'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʝ'),
        (
            ConsonantPlace::Velar,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('x'),
        (
            ConsonantPlace::Velar,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɣ'),
        (
            ConsonantPlace::Uvular,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('χ'),
        (
            ConsonantPlace::Uvular,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʁ'),
        (
            ConsonantPlace::Pharyngeal,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ħ'),
        (
            ConsonantPlace::Pharyngeal,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʕ'),
        (
            ConsonantPlace::Glotal,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('h'),
        (
            ConsonantPlace::Glotal,
            ConsonantManner::Fricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɦ'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::LateralFricative,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ɬ'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::LateralFricative,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɮ'),
        (
            ConsonantPlace::LabioDental,
            ConsonantManner::Approximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʋ'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::Approximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɹ'),
        (
            ConsonantPlace::Retroflex,
            ConsonantManner::Approximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɻ'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::Approximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('j'),
        (
            ConsonantPlace::LabioVelar,
            ConsonantManner::Approximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('w'),
        (
            ConsonantPlace::Velar,
            ConsonantManner::Approximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɰ'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::LateralApproximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('l'),
        (
            ConsonantPlace::Retroflex,
            ConsonantManner::LateralApproximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɭ'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::LateralApproximant,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ʎ'),
        (
            ConsonantPlace::Palatal,
            ConsonantManner::LateralApproximant,
            [PhoneProperty::Phonation(Phonation::Voiceless)],
        ) => Some('ʟ'),
        (
            ConsonantPlace::Alveolar,
            ConsonantManner::LateralTap,
            [PhoneProperty::Phonation(Phonation::Voiced)],
        ) => Some('ɺ'),
        _ => None,
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
        match value {
            'p' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'b' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            't' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'd' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʈ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ɖ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'c' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ɟ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'k' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'g' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'q' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ɢ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʡ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Pharyngeal,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ʔ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Glotal,
                manner: ConsonantManner::Stop,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'm' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Nasal,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɱ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Nasal,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'n' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Nasal,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɳ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Nasal,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɲ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Nasal,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ŋ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Nasal,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɴ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Nasal,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʙ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Trill,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'r' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Trill,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʀ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Trill,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ⱱ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Tap,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɾ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Tap,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɽ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Tap,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɸ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'β' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'f' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'v' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'θ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Dental,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ð' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Dental,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            's' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::SibilantFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'z' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::SibilantFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʃ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::PostAlveolar,
                manner: ConsonantManner::SibilantFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ʒ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::PostAlveolar,
                manner: ConsonantManner::SibilantFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʂ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::SibilantFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ʐ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::SibilantFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɕ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::SibilantFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ʑ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::SibilantFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ç' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ʝ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'x' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ɣ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'χ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ʁ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ħ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Pharyngeal,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ʕ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Pharyngeal,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'h' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Glotal,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ɦ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Glotal,
                manner: ConsonantManner::Fricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɬ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ɮ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralFricative,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʋ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Approximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɹ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Approximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɻ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Approximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'j' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Approximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'w' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::LabioVelar,
                manner: ConsonantManner::Approximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɰ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Approximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'l' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralApproximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ɭ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::LateralApproximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʎ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::LateralApproximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'ʟ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::LateralApproximant,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiceless)],
            })),
            'ɺ' => Ok(Phone::Consonant(Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralTap,
                properties: vec![PhoneProperty::Phonation(Phonation::Voiced)],
            })),
            'i' => Ok(Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'y' => Ok(Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ɨ' => Ok(Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Central,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ʉ' => Ok(Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Central,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ɯ' => Ok(Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Back,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'u' => Ok(Phone::Vowel(Vowel {
                height: Height::Close,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ɪ' => Ok(Phone::Vowel(Vowel {
                height: Height::NearClose,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ʏ' => Ok(Phone::Vowel(Vowel {
                height: Height::NearClose,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ʊ' => Ok(Phone::Vowel(Vowel {
                height: Height::NearClose,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'e' => Ok(Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ø' => Ok(Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ɘ' => Ok(Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Central,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ɵ' => Ok(Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Central,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ɤ' => Ok(Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Back,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'o' => Ok(Phone::Vowel(Vowel {
                height: Height::CloseMid,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ə' => Ok(Phone::Vowel(Vowel {
                height: Height::Mid,
                backness: Backness::Central,
                roundness: Roundness::Undefined,
                properties: vec![],
            })),
            'ɛ' => Ok(Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'œ' => Ok(Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ɜ' => Ok(Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Central,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ɞ' => Ok(Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Central,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ʌ' => Ok(Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Back,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ɔ' => Ok(Phone::Vowel(Vowel {
                height: Height::OpenMid,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'æ' => Ok(Phone::Vowel(Vowel {
                height: Height::NearOpen,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ɐ' => Ok(Phone::Vowel(Vowel {
                height: Height::NearOpen,
                backness: Backness::Central,
                roundness: Roundness::Undefined,
                properties: vec![],
            })),
            'a' => Ok(Phone::Vowel(Vowel {
                height: Height::Open,
                backness: Backness::Front,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ɶ' => Ok(Phone::Vowel(Vowel {
                height: Height::Open,
                backness: Backness::Front,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ɑ' => Ok(Phone::Vowel(Vowel {
                height: Height::Open,
                backness: Backness::Back,
                roundness: Roundness::UnRounded,
                properties: vec![],
            })),
            'ɒ' => Ok(Phone::Vowel(Vowel {
                height: Height::Open,
                backness: Backness::Back,
                roundness: Roundness::Rounded,
                properties: vec![],
            })),
            'ʰ' => Ok(Phone::Diacritic(PhoneProperty::ConsonantRelease(
                ConsonantRelease::Aspirated,
            ))),
            '\u{31A}' => Ok(Phone::Diacritic(PhoneProperty::ConsonantRelease(
                ConsonantRelease::NoAudible,
            ))),
            'ⁿ' => Ok(Phone::Diacritic(PhoneProperty::ConsonantRelease(
                ConsonantRelease::Nasal,
            ))),
            'ˡ' => Ok(Phone::Diacritic(PhoneProperty::ConsonantRelease(
                ConsonantRelease::Lateral,
            ))),
            'ᶿ' => Ok(Phone::Diacritic(PhoneProperty::ConsonantRelease(
                ConsonantRelease::Dental,
            ))),
            'ˣ' => Ok(Phone::Diacritic(PhoneProperty::ConsonantRelease(
                ConsonantRelease::Velar,
            ))),
            'ᵊ' => Ok(Phone::Diacritic(PhoneProperty::ConsonantRelease(
                ConsonantRelease::MidCentralVowel,
            ))),
            '\u{325}' | '\u{30A}' => Ok(Phone::Diacritic(PhoneProperty::Phonation(
                Phonation::Voiceless,
            ))),
            '\u{32C}' => Ok(Phone::Diacritic(PhoneProperty::Phonation(
                Phonation::Voiced,
            ))),
            'ʱ' | '\u{324}' => Ok(Phone::Diacritic(PhoneProperty::Phonation(
                Phonation::BreathyVoiced,
            ))),
            '\u{330}' => Ok(Phone::Diacritic(PhoneProperty::Phonation(
                Phonation::CreakyVoiced,
            ))),
            '\u{32A}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Dental,
            ))),
            '\u{33C}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Linguolabial,
            ))),
            '\u{33A}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Apical,
            ))),
            '\u{33B}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Laminal,
            ))),
            '\u{31F}' | '\u{2D6}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Advanced,
            ))),
            '\u{320}' | '\u{2D7}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Retracted,
            ))),
            '\u{308}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Centralized,
            ))),
            '\u{33D}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::MidCentralized,
            ))),
            '\u{31D}' | '\u{2D4}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Raised,
            ))),
            '\u{31E}' | '\u{2D5}' => Ok(Phone::Diacritic(PhoneProperty::Articulation(
                Articulation::Lowered,
            ))),
            '\u{339}' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::MoreRounded,
            ))),
            '\u{31C}' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::LessRounded,
            ))),
            'ʷ' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::Labialized,
            ))),
            'ʲ' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::Palatalized,
            ))),
            'ˤ' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::Pharyngealized,
            ))),
            'ˠ' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::Velarized,
            ))),
            '\u{334}' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::VelarizedOrPharyngealized,
            ))),
            '\u{318}' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::AdvancedTongueRoot,
            ))),
            '\u{319}' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::RetractedTongueRoot,
            ))),
            '\u{303}' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::Nasalized,
            ))),
            '\u{2DE}' => Ok(Phone::Diacritic(PhoneProperty::CoArticulation(
                CoArticulation::Rhoticity,
            ))),
            'ɚ' => Ok(Phone::Vowel(Vowel {
                height: Height::Mid,
                backness: Backness::Central,
                roundness: Roundness::Undefined,
                properties: vec![PhoneProperty::CoArticulation(CoArticulation::Rhoticity)],
            })),
            'ˈ' => Ok(Phone::Diacritic(PhoneProperty::Rhythm(
                Rhythm::PrimaryStress,
            ))),
            'ˌ' => Ok(Phone::Diacritic(PhoneProperty::Rhythm(
                Rhythm::SecondaryStress,
            ))),
            'ː' => Ok(Phone::Diacritic(PhoneProperty::Rhythm(Rhythm::Long))),
            'ˑ' => Ok(Phone::Diacritic(PhoneProperty::Rhythm(Rhythm::HalfLong))),
            '\u{306}' => Ok(Phone::Diacritic(PhoneProperty::Rhythm(Rhythm::ExtraShort))),
            '\u{30B}' | '˥' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::Top))),
            '\u{301}' | '˦' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::High))),
            '\u{304}' | '˧' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::Mid))),
            '\u{300}' | '˨' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::Low))),
            '\u{30F}' | '˩' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::Bottom))),
            '\u{30C}' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::Rising))),
            '\u{302}' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::Falling))),
            'ꜛ' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::Upstep))),
            'ꜜ' => Ok(Phone::Diacritic(PhoneProperty::Tone(Tone::Downstep))),
            '|' => Ok(Phone::Diacritic(PhoneProperty::Intonation(
                Intonation::MinorBreak,
            ))),
            '‖' => Ok(Phone::Diacritic(PhoneProperty::Intonation(
                Intonation::MajorBreak,
            ))),
            '↗' => Ok(Phone::Diacritic(PhoneProperty::Intonation(
                Intonation::GlobalRise,
            ))),
            '↘' => Ok(Phone::Diacritic(PhoneProperty::Intonation(
                Intonation::GlobalFall,
            ))),
            _ => Err("invalid"),
        }
    }
}

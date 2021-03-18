pub type Phones = Vec<Phone>;
use std::fmt;
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Phone {
    Consonant(Consonant),
    Vowel(Vowel),
    Diacritic(PhoneProperty),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Vowel {
    pub height: Height,
    pub backness: Backness,
    pub roundness: Roundness,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Height {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Backness {
    Front,
    Central,
    Back,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Roundness {
    UnRounded,
    Rounded,
    Undefined,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Consonant {
    pub place: ConsonantPlace,
    pub manner: ConsonantManner,
    pub phonation: Phonation,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum ConsonantPlace {
    Bilabial,
    LabioDental,
    Dental,
    Alveolar,
    PostAlveolar,
    Retroflex,
    Palatal,
    Velar,
    LabioVelar,
    Uvular,
    Pharyngeal,
    Glottal,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum ConsonantManner {
    Nasal,
    Stop,
    SibilantFricative,
    Fricative,
    Trill,
    Tap,
    LateralFricative,
    Approximant,
    LateralApproximant,
    LateralTap,
}

impl fmt::Display for ConsonantManner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ConsonantManner::Nasal => "Nasal",
            ConsonantManner::Stop => "Stop",
            ConsonantManner::SibilantFricative => "Sibilant fricative",
            ConsonantManner::Fricative => "Fricative",
            ConsonantManner::Trill => "Trill",
            ConsonantManner::Tap => "Tap",
            ConsonantManner::LateralFricative => "Lateral fricative",
            ConsonantManner::Approximant => "Approximant",
            ConsonantManner::LateralApproximant => "Lateral approximant",
            ConsonantManner::LateralTap => "Lateral tap",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Height::*;
        let s = match self {
            Close => "Close",
            NearClose => "Near close",
            CloseMid => "Close mid",
            Mid => "Mid",
            OpenMid => "Open mid",
            NearOpen => "Near open",
            Open => "Open",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(C)]
pub enum PhoneProperty {
    ConsonantRelease(ConsonantRelease),
    Phonation(Phonation),
    Articulation(Articulation),
    CoArticulation(CoArticulation),
    Rhythm(Rhythm),
    Tone(Tone),
    Intonation(Intonation),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum ConsonantRelease {
    Aspirated,
    NoAudible,
    Nasal,
    Lateral,
    Dental,
    Velar,
    MidCentralVowel,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Phonation {
    Voiceless,
    Voiced,
    BreathyVoiced,
    CreakyVoiced,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Rhythm {
    PrimaryStress,
    SecondaryStress,
    Long,
    HalfLong,
    ExtraShort,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Intonation {
    MinorBreak,
    MajorBreak,
    GlobalRise,
    GlobalFall,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
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

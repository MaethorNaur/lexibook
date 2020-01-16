pub type Phones = Vec<Phone>;

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Copy)]
pub enum Phone {
    Consonant(Consonant),
    Vowel(Vowel),
    Diacritic(PhoneProperty),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Copy)]
pub struct Vowel {
    pub height: Height,
    pub backness: Backness,
    pub roundness: Roundness,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Backness {
    Front,
    Central,
    Back,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Roundness {
    UnRounded,
    Rounded,
    Undefined,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Copy)]
pub struct Consonant {
    pub place: ConsonantPlace,
    pub manner: ConsonantManner,
    pub phonation: Phonation,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum ConsonantManner {
    Stop,
    SibilantFricative,
    Fricative,
    Nasal,
    Trill,
    Tap,
    LateralFricative,
    Approximant,
    LateralApproximant,
    LateralTap,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Phonation {
    Voiceless,
    Voiced,
    BreathyVoiced,
    CreakyVoiced,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Rhythm {
    PrimaryStress,
    SecondaryStress,
    Long,
    HalfLong,
    ExtraShort,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
#[repr(u8)]
pub enum Intonation {
    MinorBreak,
    MajorBreak,
    GlobalRise,
    GlobalFall,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Ord, PartialOrd, Copy)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;
    #[test]
    fn test_manner_order() {
        let result = ConsonantManner::Nasal.cmp(&ConsonantManner::Stop);
        assert_eq!(result, Ordering::Greater)
    }
}

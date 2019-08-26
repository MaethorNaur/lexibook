use std::convert::{From, Into};
use Articulation::*;
use Characteristic::*;
use Type::*;

#[derive(Debug)]
pub enum Syllable<'a> {
    Optional,
    Sounds(&'a str),
    Class(char),
}
#[derive(Debug)]
pub enum Characteristic {
    Voiced,
    Unvoiced,
    Rounded,
    UnRounded,
    None,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Consonant,
    Vowel,
    Unknown,
}

#[derive(EnumIter, Debug)]
pub enum Articulation {
    Bilabial,
    LabioDental,
    Dental,
    Alveolar,
    PostAlveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glotal,
    Plosive,
    Nasal,
    Trill,
    Tap,
    Fricative,
    LateralFricative,
    Approximant,
    LateralApproximant,
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
    Front,
    Central,
    Back,
}

#[derive(Debug)]
pub struct IPASound {
    pub type_of_sound: Type,
    pub character: char,
    pub articulations: Vec<Articulation>,
    pub characteristic: Characteristic,
}

impl From<char> for IPASound {
    fn from(value: char) -> Self {
        match value {
            'p' => IPASound {
                type_of_sound: Consonant,
                character: 'p',
                articulations: vec![Bilabial, Plosive],
                characteristic: Unvoiced,
            },
            'b' => IPASound {
                type_of_sound: Consonant,
                character: 'b',
                articulations: vec![Bilabial, Plosive],
                characteristic: Voiced,
            },
            't' => IPASound {
                type_of_sound: Consonant,
                character: 't',
                articulations: vec![Dental, Alveolar, PostAlveolar, Plosive],
                characteristic: Unvoiced,
            },
            'd' => IPASound {
                type_of_sound: Consonant,
                character: 'd',
                articulations: vec![Dental, Alveolar, PostAlveolar, Plosive],
                characteristic: Voiced,
            },
            'ʈ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʈ',
                articulations: vec![Retroflex, Plosive],
                characteristic: Unvoiced,
            },
            'ɖ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɖ',
                articulations: vec![Retroflex, Plosive],
                characteristic: Voiced,
            },
            'c' => IPASound {
                type_of_sound: Consonant,
                character: 'c',
                articulations: vec![Palatal, Plosive],
                characteristic: Unvoiced,
            },
            'ɟ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɟ',
                articulations: vec![Palatal, Plosive],
                characteristic: Voiced,
            },
            'k' => IPASound {
                type_of_sound: Consonant,
                character: 'k',
                articulations: vec![Velar, Plosive],
                characteristic: Unvoiced,
            },
            'g' => IPASound {
                type_of_sound: Consonant,
                character: 'g',
                articulations: vec![Velar, Plosive],
                characteristic: Voiced,
            },
            'q' => IPASound {
                type_of_sound: Consonant,
                character: 'q',
                articulations: vec![Uvular, Plosive],
                characteristic: Unvoiced,
            },
            'ɢ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɢ',
                articulations: vec![Uvular, Plosive],
                characteristic: Voiced,
            },
            'ʔ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʔ',
                articulations: vec![Glotal, Plosive],
                characteristic: Unvoiced,
            },
            'm' => IPASound {
                type_of_sound: Consonant,
                character: 'm',
                articulations: vec![Bilabial, Nasal],
                characteristic: Voiced,
            },
            'ɱ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɱ',
                articulations: vec![LabioDental, Nasal],
                characteristic: Voiced,
            },
            'n' => IPASound {
                type_of_sound: Consonant,
                character: 'n',
                articulations: vec![Dental, Alveolar, PostAlveolar, Nasal],
                characteristic: Voiced,
            },
            'ɳ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɳ',
                articulations: vec![Retroflex, Nasal],
                characteristic: Voiced,
            },
            'ɲ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɲ',
                articulations: vec![Palatal, Nasal],
                characteristic: Voiced,
            },
            'ŋ' => IPASound {
                type_of_sound: Consonant,
                character: 'ŋ',
                articulations: vec![Velar, Nasal],
                characteristic: Voiced,
            },
            'ɴ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɴ',
                articulations: vec![Uvular, Nasal],
                characteristic: Voiced,
            },
            'ʙ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʙ',
                articulations: vec![Bilabial, Trill],
                characteristic: Voiced,
            },
            'r' => IPASound {
                type_of_sound: Consonant,
                character: 'r',
                articulations: vec![Dental, Alveolar, PostAlveolar, Trill],
                characteristic: Voiced,
            },
            'ʀ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʀ',
                articulations: vec![Uvular, Trill],
                characteristic: Voiced,
            },
            'ɾ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɾ',
                articulations: vec![Dental, Alveolar, PostAlveolar, Tap],
                characteristic: Voiced,
            },
            'ɽ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɽ',
                articulations: vec![Retroflex, Tap],
                characteristic: Voiced,
            },
            'ɸ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɸ',
                articulations: vec![Bilabial, Fricative],
                characteristic: Unvoiced,
            },
            'β' => IPASound {
                type_of_sound: Consonant,
                character: 'β',
                articulations: vec![Bilabial, Fricative],
                characteristic: Voiced,
            },
            'f' => IPASound {
                type_of_sound: Consonant,
                character: 'f',
                articulations: vec![LabioDental, Fricative],
                characteristic: Unvoiced,
            },
            'v' => IPASound {
                type_of_sound: Consonant,
                character: 'v',
                articulations: vec![LabioDental, Fricative],
                characteristic: Voiced,
            },
            'θ' => IPASound {
                type_of_sound: Consonant,
                character: 'θ',
                articulations: vec![Dental, Fricative],
                characteristic: Unvoiced,
            },
            'ð' => IPASound {
                type_of_sound: Consonant,
                character: 'ð',
                articulations: vec![Dental, Fricative],
                characteristic: Voiced,
            },
            's' => IPASound {
                type_of_sound: Consonant,
                character: 's',
                articulations: vec![Alveolar, Fricative],
                characteristic: Unvoiced,
            },
            'z' => IPASound {
                type_of_sound: Consonant,
                character: 'z',
                articulations: vec![Alveolar, Fricative],
                characteristic: Voiced,
            },
            'ʃ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʃ',
                articulations: vec![PostAlveolar, Fricative],
                characteristic: Unvoiced,
            },
            'ʒ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʒ',
                articulations: vec![PostAlveolar, Fricative],
                characteristic: Voiced,
            },
            'ʂ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʂ',
                articulations: vec![Retroflex, Fricative],
                characteristic: Unvoiced,
            },
            'ʐ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʐ',
                articulations: vec![Retroflex, Fricative],
                characteristic: Voiced,
            },
            'ç' => IPASound {
                type_of_sound: Consonant,
                character: 'ç',
                articulations: vec![Palatal, Fricative],
                characteristic: Unvoiced,
            },
            'ʝ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʝ',
                articulations: vec![Palatal, Fricative],
                characteristic: Voiced,
            },
            'x' => IPASound {
                type_of_sound: Consonant,
                character: 'x',
                articulations: vec![Velar, Fricative],
                characteristic: Unvoiced,
            },
            'ɣ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɣ',
                articulations: vec![Velar, Fricative],
                characteristic: Voiced,
            },
            'χ' => IPASound {
                type_of_sound: Consonant,
                character: 'χ',
                articulations: vec![Uvular, Fricative],
                characteristic: Unvoiced,
            },
            'ʁ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʁ',
                articulations: vec![Uvular, Fricative],
                characteristic: Voiced,
            },
            'ħ' => IPASound {
                type_of_sound: Consonant,
                character: 'ħ',
                articulations: vec![Pharyngeal, Fricative],
                characteristic: Unvoiced,
            },
            'ʕ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʕ',
                articulations: vec![Pharyngeal, Fricative],
                characteristic: Voiced,
            },
            'h' => IPASound {
                type_of_sound: Consonant,
                character: 'h',
                articulations: vec![Glotal, Fricative],
                characteristic: Unvoiced,
            },
            'ɦ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɦ',
                articulations: vec![Glotal, Fricative],
                characteristic: Voiced,
            },
            'ɬ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɬ',
                articulations: vec![Dental, Alveolar, PostAlveolar, LateralFricative],
                characteristic: Unvoiced,
            },
            'ɮ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɮ',
                articulations: vec![Dental, Alveolar, PostAlveolar, LateralFricative],
                characteristic: Voiced,
            },
            'ʋ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʋ',
                articulations: vec![LabioDental, Approximant],
                characteristic: Voiced,
            },
            'ɹ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɹ',
                articulations: vec![Dental, Alveolar, PostAlveolar, Approximant],
                characteristic: Voiced,
            },
            'ɻ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɻ',
                articulations: vec![Retroflex, Approximant],
                characteristic: Voiced,
            },
            'j' => IPASound {
                type_of_sound: Consonant,
                character: 'j',
                articulations: vec![Palatal, Approximant],
                characteristic: Voiced,
            },
            'ɰ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɰ',
                articulations: vec![Velar, Approximant],
                characteristic: Voiced,
            },
            'l' => IPASound {
                type_of_sound: Consonant,
                character: 'l',
                articulations: vec![Dental, Alveolar, PostAlveolar, LateralApproximant],
                characteristic: Voiced,
            },
            'ɭ' => IPASound {
                type_of_sound: Consonant,
                character: 'ɭ',
                articulations: vec![Retroflex, LateralApproximant],
                characteristic: Voiced,
            },
            'ʎ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʎ',
                articulations: vec![Palatal, LateralApproximant],
                characteristic: Voiced,
            },
            'ʟ' => IPASound {
                type_of_sound: Consonant,
                character: 'ʟ',
                articulations: vec![Velar, LateralApproximant],
                characteristic: Voiced,
            },

            'i' => IPASound {
                type_of_sound: Vowel,
                character: 'i',
                articulations: vec![Close, Front],
                characteristic: UnRounded,
            },
            'y' => IPASound {
                type_of_sound: Vowel,
                character: 'i',
                articulations: vec![Close, Front],
                characteristic: Rounded,
            },
            'ɨ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɨ',
                articulations: vec![Close, Central],
                characteristic: UnRounded,
            },
            'ʉ' => IPASound {
                type_of_sound: Vowel,
                character: 'ʉ',
                articulations: vec![Close, Central],
                characteristic: Rounded,
            },
            'ɯ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɯ',
                articulations: vec![Close, Back],
                characteristic: UnRounded,
            },
            'u' => IPASound {
                type_of_sound: Vowel,
                character: 'u',
                articulations: vec![Close, Back],
                characteristic: Rounded,
            },

            'ɪ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɪ',
                articulations: vec![NearClose, Front],
                characteristic: UnRounded,
            },
            'ʏ' => IPASound {
                type_of_sound: Vowel,
                character: 'ʏ',
                articulations: vec![NearClose, Front],
                characteristic: Rounded,
            },
            'ʊ' => IPASound {
                type_of_sound: Vowel,
                character: 'ʊ',
                articulations: vec![NearClose, Back],
                characteristic: Rounded,
            },

            'e' => IPASound {
                type_of_sound: Vowel,
                character: 'e',
                articulations: vec![CloseMid, Front],
                characteristic: UnRounded,
            },
            'ø' => IPASound {
                type_of_sound: Vowel,
                character: 'ø',
                articulations: vec![CloseMid, Front],
                characteristic: Rounded,
            },
            'ɘ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɘ',
                articulations: vec![CloseMid, Central],
                characteristic: UnRounded,
            },
            'ɵ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɵ',
                articulations: vec![CloseMid, Central],
                characteristic: Rounded,
            },
            'ɤ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɤ',
                articulations: vec![CloseMid, Back],
                characteristic: UnRounded,
            },
            'o' => IPASound {
                type_of_sound: Vowel,
                character: 'o',
                articulations: vec![CloseMid, Back],
                characteristic: Rounded,
            },

            'ə' => IPASound {
                type_of_sound: Vowel,
                character: 'ə',
                articulations: vec![Mid, Central],
                characteristic: Characteristic::None,
            },

            'ɛ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɛ',
                articulations: vec![OpenMid, Front],
                characteristic: UnRounded,
            },
            'œ' => IPASound {
                type_of_sound: Vowel,
                character: 'œ',
                articulations: vec![OpenMid, Front],
                characteristic: Rounded,
            },
            'ɜ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɜ',
                articulations: vec![OpenMid, Central],
                characteristic: UnRounded,
            },
            'ɞ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɞ',
                articulations: vec![OpenMid, Central],
                characteristic: Rounded,
            },
            'ʌ' => IPASound {
                type_of_sound: Vowel,
                character: 'ʌ',
                articulations: vec![OpenMid, Back],
                characteristic: UnRounded,
            },
            'ɔ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɔ',
                articulations: vec![OpenMid, Back],
                characteristic: Rounded,
            },

            'æ' => IPASound {
                type_of_sound: Vowel,
                character: 'æ',
                articulations: vec![NearOpen, Front],
                characteristic: UnRounded,
            },
            'ɐ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɐ',
                articulations: vec![NearOpen, Central],
                characteristic: Characteristic::None,
            },

            'a' => IPASound {
                type_of_sound: Vowel,
                character: 'a',
                articulations: vec![Open, Front],
                characteristic: UnRounded,
            },
            'ɶ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɶ',
                articulations: vec![Open, Front],
                characteristic: Rounded,
            },
            'ɑ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɑ',
                articulations: vec![Open, Back],
                characteristic: UnRounded,
            },
            'ɒ' => IPASound {
                type_of_sound: Vowel,
                character: 'ɒ',
                articulations: vec![Open, Back],
                characteristic: Rounded,
            },
            unknown => IPASound {
                type_of_sound: Unknown,
                character: unknown,
                articulations: vec![],
                characteristic: Characteristic::None,
            },
        }
    }
}
impl Into<char> for IPASound {
    fn into(self) -> char {
        self.character
    }
}

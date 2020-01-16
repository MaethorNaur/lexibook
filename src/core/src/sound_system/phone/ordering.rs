use super::*;
use std::cmp::Ordering;

impl PartialOrd for Phone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = match (self, other) {
            (Phone::Consonant(left), Phone::Consonant(right)) => compare_consonant(*left, *right),
            (Phone::Vowel(left), Phone::Vowel(right)) => compare_vowel(*left, *right),
            (Phone::Diacritic(_), Phone::Diacritic(_)) => Ordering::Equal,
            (Phone::Consonant(_), _) => Ordering::Greater,
            (Phone::Vowel(_), Phone::Diacritic(_)) => Ordering::Greater,
            _ => Ordering::Less,
        };
        Some(res)
    }
}

fn compare_vowel(left: Vowel, right: Vowel) -> Ordering {
    match (left.pair(), right.pair()) {
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        _ => right
            .height
            .cmp(&left.height)
            .then(right.backness.cmp(&left.backness))
            .then(right.roundness.cmp(&left.roundness)),
    }
}

impl Vowel {
    pub fn pair(self) -> Option<Self> {
        self.roundness.pair().and_then(|opposite| match self {
            Vowel {
                height, backness, ..
            } if height != Height::NearClose
                && height != Height::Mid
                && height != Height::NearOpen =>
            {
                Some(Vowel {
                    height,
                    backness,
                    roundness: opposite,
                })
            }

            Vowel {
                height: Height::NearClose,
                backness: Backness::Front,
                ..
            } => Some(Vowel {
                height: Height::NearClose,
                backness: Backness::Front,
                roundness: opposite,
            }),
            _ => None,
        })
    }
}

impl Roundness {
    fn pair(self) -> Option<Self> {
        match self {
            Roundness::Rounded => Some(Roundness::UnRounded),
            Roundness::UnRounded => Some(Roundness::Rounded),
            _ => None,
        }
    }
}

impl Consonant {
    fn pair(self) -> Option<Self> {
        self.phonation.pair().and_then(|opposite| match self {
            Consonant {
                place: ConsonantPlace::Pharyngeal,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }
            | Consonant {
                place: ConsonantPlace::Glotal,
                manner: ConsonantManner::Stop,
                phonation: Phonation::Voiceless,
            }
            | Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Nasal,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Bilabial,
                manner: ConsonantManner::Trill,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Trill,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Uvular,
                manner: ConsonantManner::Trill,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Tap,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Tap,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Tap,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::LabioDental,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Palatal,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::LabioVelar,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Velar,
                manner: ConsonantManner::Approximant,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralApproximant,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Retroflex,
                manner: ConsonantManner::LateralApproximant,
                phonation: Phonation::Voiced,
            }
            | Consonant {
                place: ConsonantPlace::Alveolar,
                manner: ConsonantManner::LateralTap,
                phonation: Phonation::Voiced,
            } => None,
            Consonant { place, manner, .. } => Some(Consonant {
                place,
                manner,
                phonation: opposite,
            }),
        })
    }
}

impl Phonation {
    fn pair(self) -> Option<Self> {
        match self {
            Phonation::Voiced => Some(Phonation::Voiceless),
            Phonation::Voiceless => Some(Phonation::Voiced),
            _ => None,
        }
    }
}
fn compare_consonant(left: Consonant, right: Consonant) -> Ordering {
    match (left.pair(), right.pair()) {
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        _ => right
            .manner
            .cmp(&left.manner)
            .then(right.place.cmp(&left.place))
            .then(right.phonation.cmp(&left.phonation)),
    }
}

impl Ord for Phone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Less)
    }
}

use super::*;

impl Phone {
    pub fn classes(&self) -> Option<Vec<&str>> {
        match self {
            Phone::Vowel(Vowel {
                height,
                backness,
                roundness,
            }) => {
                let mut result = vec!["V"];
                match height {
                    Height::Close => result.push("H"),
                    Height::CloseMid | Height::NearClose => result.push("E"),
                    Height::Mid | Height::OpenMid => result.push("O"),
                    _ => result.push("L"),
                }
                match backness {
                    Backness::Front => result.push("F"),
                    Backness::Central => result.push("A"),
                    Backness::Back => result.push("B"),
                }
                match roundness {
                    Roundness::UnRounded => result.push("U"),
                    Roundness::Rounded => result.push("O"),
                    Roundness::Undefined => result.push("Y"),
                }
                Some(result)
            }
            Phone::Consonant(Consonant {
                place,
                manner,
                phonation,
            }) => {
                let mut result = vec!["C"];
                match place {
                    ConsonantPlace::Bilabial
                    | ConsonantPlace::LabioDental
                    | ConsonantPlace::LabioVelar => result.push("M"),
                    ConsonantPlace::Dental
                    | ConsonantPlace::Alveolar
                    | ConsonantPlace::PostAlveolar => result.push("D"),
                    ConsonantPlace::Retroflex => result.push("T"),
                    ConsonantPlace::Palatal => result.push("J"),
                    ConsonantPlace::Velar => result.push("G"),
                    ConsonantPlace::Uvular => result.push("Q"),
                    ConsonantPlace::Pharyngeal | ConsonantPlace::Glotal => result.push("T"),
                }
                match manner {
                    ConsonantManner::Stop => result.push("P"),
                    ConsonantManner::Fricative
                    | ConsonantManner::SibilantFricative
                    | ConsonantManner::LateralFricative => result.push("X"),
                    ConsonantManner::Nasal => result.push("N"),
                    ConsonantManner::Trill | ConsonantManner::Tap | ConsonantManner::LateralTap => {
                        result.push("N")
                    }
                    ConsonantManner::Approximant | ConsonantManner::LateralApproximant => {
                        result.push("W")
                    }
                }
                match phonation {
                    Phonation::Voiced => result.push("Z"),
                    _ => result.push("S"),
                }
                Some(result)
            }
            _ => None,
        }
    }
}

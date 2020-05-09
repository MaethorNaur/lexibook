use super::{Rule, SoundSystem};
use regex::{Captures, Regex};

#[derive(Debug, Default, Serialize, Eq, PartialEq)]
pub struct Transformation {
    pub output: Vec<String>,
    pub history: Vec<History>,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct History {
    pub rule: String,
    pub words: Vec<String>,
}

impl SoundSystem {
    pub fn sound_trasformation(&mut self, words: Vec<String>) -> Transformation {
        let mut history: Vec<History> = vec![];
        let output = self
            .rules()
            .clone()
            .iter()
            .fold(words, |words, rule| match rule {
                Rule::SoundRule {
                    name,
                    regex,
                    replacement,
                } => {
                    let regex = Regex::new(&regex).unwrap();
                    let output = words
                        .iter()
                        .map(|word| apply_sound_rule(self, &regex, replacement.as_ref(), word))
                        .collect::<Vec<_>>();
                    history.push(History {
                        rule: name.to_string(),
                        words: output.clone(),
                    });

                    output
                }
                Rule::PhonemeRule {
                    name,
                    phoneme_differences,
                } => {
                    self.update_phoneme(phoneme_differences);
                    history.push(History {
                        rule: name.to_string(),
                        words: words.clone(),
                    });
                    words
                }
            });
        Transformation { output, history }
    }
}

fn apply_sound_rule(
    sound_system: &SoundSystem,
    rule: &Regex,
    replacement: Option<&String>,
    word: &'_ str,
) -> String {
    let capture_names = rule
        .capture_names()
        .enumerate()
        .filter_map(|(idx, c)| c.map(|c| (idx, c)))
        .collect::<Vec<_>>();

    let result = rule.replace_all(word, |capture: &Captures| {
        let mut result = String::new();
        let first = capture_names.first().map(|t| t.0).unwrap();
        let last = capture_names.last().map(|t| t.0).unwrap();
        for i in 1..first {
            result.push_str(&capture[i]);
        }

        match capture_names.len() {
            1 => {
                if let Some(replacement_value) = replacement {
                    result.push_str(&replacement_value)
                }
            }
            _ => {
                let to_replace = capture_names
                    .iter()
                    .filter(|(_idx, c)| !c.eq_ignore_ascii_case("input"))
                    .map(|(_idx, class)| {
                        sound_system
                            .classes()
                            .get(&(*class).to_string())
                            .and_then(|letters| {
                                let letter = capture.name(class).unwrap().as_str();
                                letters
                                    .iter()
                                    .position(|l| *l == letter)
                                    .map(|position| (letter, position))
                            })
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                let mut i = 0;
                replacement
                    .map(|value| {
                        value.chars().for_each(|c| match c {
                            c if c.is_uppercase() => {
                                if let Some((original_letter, position)) = to_replace.get(i) {
                                    let class_name = c.to_string();
                                    match sound_system
                                        .classes()
                                        .get(&class_name as &str)
                                        .and_then(|letters| letters.get(*position))
                                    {
                                        None => {
                                            trace!("replacement letter not found");
                                            result.push_str(original_letter)
                                        }

                                        Some(letter) => {
                                            trace!("replacement letter: {:#?}", letter);
                                            result.push_str(letter);
                                            i += 1;
                                        }
                                    }
                                }
                            }
                            _ => result.push(c),
                        })
                    })
                    .unwrap_or_else(|| ());
            }
        }
        for i in last + 1..capture.len() {
            result.push_str(&capture[i]);
        }
        result
    });
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sound_system::{Rule, SoundSystem};
    use std::collections::HashMap;
    fn create_sound_system(classes: Vec<(String, Vec<String>)>, rules: Vec<Rule>) -> SoundSystem {
        SoundSystem::new(
            classes.iter().cloned().collect(),
            HashMap::new(),
            vec![],
            vec![],
            rules,
        )
    }

    #[test]
    fn test_expand() {
        let mut sound_system = create_sound_system(
            vec![
                (
                    "S".to_string(),
                    vec!["p".to_string(), "t".to_string(), "c".to_string()],
                ),
                (
                    "Z".to_string(),
                    vec!["b".to_string(), "d".to_string(), "g".to_string()],
                ),
            ],
            vec![Rule::SoundRule {
                name: "V_*V: S -> Z".to_string(),
                regex:
                    "(a|e|o|y|ä|wa|ë|we|ö|wo)(?P<input>(?P<S>p|t|g))(.*?)(a|e|o|y|ä|wa|ë|we|ö|wo)"
                        .to_string(),
                replacement: Some("Z".to_string()),
            }],
        );
        let words = vec!["apxal".to_string()];
        let result = sound_system.sound_trasformation(words);
        assert_eq!(
            result,
            Transformation {
                output: vec!["abxal".to_string()],
                history: vec![History {
                    rule: "V_*V: S -> Z".to_string(),
                    words: vec!["abxal".to_string()],
                }]
            }
        )
    }
    #[test]
    fn test_replace_class_to_letter() {
        let mut sound_system = create_sound_system(
            vec![(
                "S".to_string(),
                vec!["p".to_string(), "t".to_string(), "c".to_string()],
            )],
            vec![Rule::SoundRule {
                name: "V_V: S -> x".to_string(),
                regex:
                    "(a|e|o|y|ja|wa|je|we|jo|wo)(?P<input>(?P<S>p|t|c))(a|e|o|y|ja|wa|je|we|jo|wo)"
                        .to_string(),
                replacement: Some("x".to_string()),
            }],
        );
        let words = vec!["apaepal".to_string()];
        let result = sound_system.sound_trasformation(words);
        assert_eq!(
            result,
            Transformation {
                output: vec!["axaexal".to_string()],
                history: vec![History {
                    rule: "V_V: S -> x".to_string(),
                    words: vec!["axaexal".to_string()],
                }]
            }
        )
    }

    #[test]
    fn test_unknown_replacement_class() {
        let mut sound_system = create_sound_system(
            vec![(
                "S".to_string(),
                vec!["p".to_string(), "t".to_string(), "c".to_string()],
            )],
            vec![Rule::SoundRule {
                name: "V_V: S -> Z".to_string(),
                regex:
                    "(a|e|o|y|ja|wa|je|we|jo|wo)(?P<input>(?P<S>p|t|c))(a|e|o|y|ja|wa|je|we|jo|wo)"
                        .to_string(),
                replacement: Some("Z".to_string()),
            }],
        );
        let words = vec!["apal".to_string()];
        let result = sound_system.sound_trasformation(words);
        assert_eq!(
            result,
            Transformation {
                output: vec!["apal".to_string()],
                history: vec![History {
                    rule: "V_V: S -> Z".to_string(),
                    words: vec!["apal".to_string()],
                }]
            }
        )
    }

    #[test]
    fn test_sound_transformation() {
        let mut sound_system = create_sound_system(
        vec![
            ("S".to_string(), vec!["p".to_string(),"t".to_string(),"c".to_string()]),
            ("Z".to_string(),vec!["b".to_string(),"d".to_string(),"g".to_string()])
        ],
        vec![
        Rule::SoundRule{
            name: "#_: l -> ".to_string(),
            regex: "^(?P<input>l)".to_string(),
            replacement: None,
        },
        Rule::SoundRule{
            name: "_#: l -> ".to_string(),
            regex: "(?P<input>l)$".to_string(),
            replacement: None,
        },
        Rule::SoundRule{
            name: "V_V: S -> Z".to_string(), 
            regex: "(a|e|o|y|ja|wa|je|we|jo|wo)(?P<input>(?P<S>p|t|c))(a|e|o|y|ja|wa|je|we|jo|wo)".to_string(),
            replacement: Some("Z".to_string()),
        }
        ]);

        let words = vec!["la".to_string(), "apaacal".to_string()];
        let result = sound_system.sound_trasformation(words);
        assert_eq!(
            result,
            Transformation {
                output: vec!["a".to_string(), "abaaga".to_string()],
                history: vec![
                    History {
                        rule: "#_: l -> ".to_string(),
                        words: vec!["a".to_string(), "apaacal".to_string()],
                    },
                    History {
                        rule: "_#: l -> ".to_string(),
                        words: vec!["a".to_string(), "apaaca".to_string()],
                    },
                    History {
                        rule: "V_V: S -> Z".to_string(),
                        words: vec!["a".to_string(), "abaaga".to_string()],
                    }
                ]
            }
        )
    }
}

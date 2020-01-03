use crate::sound_system::SoundSystem;
use regex::{Captures, Regex};

#[derive(Debug, Default, Serialize)]
pub struct Transformation {
    pub output: Vec<String>,
    pub history: Vec<(String, Vec<String>)>,
}

pub fn sound_trasformation(sound_system: &SoundSystem, words: Vec<String>) -> Transformation {
    let mut history: Vec<(String, Vec<String>)> = vec![];
    let output = sound_system
        .rules
        .iter()
        .fold(words, |words, (name, rule, replacement)| {
            let mut output = vec![];
            for word in &words {
                output.push(apply_rule(
                    sound_system,
                    Regex::new(&rule).unwrap(),
                    replacement,
                    word,
                ));
            }
            history.push((name.to_string(), output.clone()));
            output
        });
    Transformation { output, history }
}

fn apply_rule(
    sound_system: &SoundSystem,
    rule: Regex,
    replacement: &Option<&'_ str>,
    word: &'_ str,
) -> String {
    let capture_names = rule.capture_names().filter_map(|c| c).collect::<Vec<_>>();
    let result = rule.replace_all(word, |capture: &Captures| {
        let mut result = String::from(&capture[1]);
        match capture_names.len() {
            1 => result.push_str(replacement.unwrap_or("")),
            _ => {
                let to_replace = capture_names
                    .iter()
                    .filter(|c| !c.eq_ignore_ascii_case("input"))
                    .map(|class| {
                        sound_system
                            .classes
                            .get(class)
                            .and_then(|letters| {
                                let letter = capture.name(class).unwrap().as_str();
                                letters.iter().position(|l| *l == letter)
                            })
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                trace!("{:#?}", to_replace);
                let mut i = 0;
                replacement
                    .map(|value| {
                        value.chars().for_each(|c| match c {
                            c if c.is_uppercase() => {
                                if let Some(position) = to_replace.get(i) {
                                    let class_name = c.to_string();
                                    if let Some(letters) =
                                        sound_system.classes.get(&class_name as &str)
                                    {
                                        if let Some(letter) = letters.get(*position) {
                                            debug!("{:#?}", letter);
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
        result.push_str(&capture[capture.len() - 1]);
        result
    });
    result.to_string()
}

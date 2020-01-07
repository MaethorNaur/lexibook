use std::collections::HashMap;
use std::fmt;

use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "wgl.pest"]
struct WGLParser;

#[derive(Debug)]
pub struct AST<'a> {
    pub imports: Vec<&'a str>,
    pub letters: Vec<Letter<'a>>,
    pub classes: HashMap<&'a str, Vec<&'a str>>,
    pub syllables: Vec<Vec<&'a str>>,
    pub rules: Vec<TransformationRule<'a>>,
}

#[derive(Debug)]
pub enum Letter<'a> {
    WithPhoneticNotation(&'a str, Vec<&'a str>),
    OnlyRepresensation(&'a str),
}

#[derive(Debug)]
pub enum TransformationRule<'a> {
    SoundRule(SoundRule<'a>),
    PhonemeRule((&'a str, Option<&'a str>)),
}

#[derive(Debug)]
pub struct SoundRule<'a> {
    environment: Environment<'a>,
    input: &'a str,
    output: Option<&'a str>,
}

#[derive(Debug)]
pub enum Environment<'a> {
    All,
    Match(&'a str),
}

impl<'a> TransformationRule<'a> {
    pub fn environment(&self) -> Option<&Environment<'a>> {
        match self {
            TransformationRule::SoundRule(rule) => Some(&rule.environment),
            TransformationRule::PhonemeRule(_rule) => None,
        }
    }

    pub fn input(&self) -> &'a str {
        match self {
            TransformationRule::SoundRule(rule) => rule.input,
            TransformationRule::PhonemeRule(rule) => rule.0,
        }
    }

    pub fn output(&self) -> Option<&'a str> {
        match self {
            TransformationRule::SoundRule(rule) => rule.output,
            TransformationRule::PhonemeRule(rule) => rule.1,
        }
    }
}

impl<'a> fmt::Display for TransformationRule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode = match self {
            TransformationRule::SoundRule(_rule) => "->",
            TransformationRule::PhonemeRule(_rule) => "~>",
        };

        write!(
            f,
            "{}{} {} {}",
            self.environment()
                .map(|e| {
                    let mut s = String::from(e.to_string());
                    s.push_str(" : ");
                    s
                })
                .unwrap_or_else(|| "".to_string()),
            self.input(),
            mode,
            self.output().unwrap_or("")
        )
    }
}
impl<'a> Environment<'a> {
    pub fn to_string(&self) -> &'a str {
        match self {
            Environment::All => "_",
            Environment::Match(s) => s,
        }
    }
}

impl<'a> Letter<'a> {
    pub fn to_string(&self) -> &'a str {
        match self {
            Letter::OnlyRepresensation(s) => s,
            Letter::WithPhoneticNotation(s, _) => s,
        }
    }
}

pub fn from_string(input: &'_ str) -> Result<AST, Error<Rule>> {
    let pairs = WGLParser::parse(Rule::wgl, &input)?;
    trace!("WGL parsed AST: {:#?}", pairs);
    let mut ast = AST {
        imports: vec![],
        letters: vec![],
        classes: HashMap::new(),
        syllables: vec![],
        rules: vec![],
    };
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::import => ast.imports = build_imports(pair),
            Rule::letters => ast.letters = build_letters(pair),
            Rule::class => {
                let (name, values) = build_class(pair);
                ast.classes.insert(name, values);
            }
            Rule::syllables => ast.syllables = build_syllables(pair),
            Rule::rules => ast.rules = build_rules(pair),
            _ => {}
        }
    }

    trace!("AST: {:#?}", ast);

    Ok(ast)
}

fn build_imports(pair: pest::iterators::Pair<'_, Rule>) -> Vec<&'_ str> {
    pair.into_inner().map(|r| r.as_str()).collect()
}

#[allow(irrefutable_let_patterns)]
fn build_letters(pair: pest::iterators::Pair<'_, Rule>) -> Vec<Letter<'_>> {
    let pairs = pair.into_inner();
    let mut letters = vec![];
    for pair in pairs {
        if let _ = Rule::letter_sound {
            let mut letter_sound = pair.into_inner();
            let represensation = letter_sound.next().unwrap().as_str();
            let sounds: Vec<_> = letter_sound.map(build_sound).collect();
            let letter = if sounds.is_empty() {
                Letter::OnlyRepresensation(represensation)
            } else {
                Letter::WithPhoneticNotation(represensation, sounds)
            };
            letters.push(letter)
        }
    }
    letters
}

fn build_rules(pair: pest::iterators::Pair<'_, Rule>) -> Vec<TransformationRule<'_>> {
    pair.into_inner()
        .filter_map(build_sound_or_phoneme_rule)
        .collect()
}

fn build_sound_or_phoneme_rule(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Option<TransformationRule<'_>> {
    let rule = pair.into_inner().next().unwrap();
    match rule.as_rule() {
        Rule::sound_rule => Some(TransformationRule::SoundRule(build_sound_rule(rule))),
        Rule::phoneme_rule => Some(TransformationRule::PhonemeRule(build_phoneme_rule(rule))),
        _ => None,
    }
}

fn build_phoneme_rule(pair: pest::iterators::Pair<'_, Rule>) -> (&'_ str, Option<&'_ str>) {
    let mut pairs = pair.into_inner();
    let input = pairs.next().map(build_input_output).unwrap();
    let sounds = pairs.as_str();
    let output = if sounds.is_empty() {
        None
    } else {
        Some(sounds)
    };

    (input, output)
}

fn build_sound_rule(pair: pest::iterators::Pair<'_, Rule>) -> SoundRule<'_> {
    let mut pairs = pair.into_inner();
    let environment = pairs.next().map(build_environment).unwrap();
    let input = pairs.next().map(build_input_output).unwrap();
    let output = pairs.next().map(build_input_output);
    SoundRule {
        environment,
        input,
        output,
    }
}

fn build_environment(pair: pest::iterators::Pair<'_, Rule>) -> Environment<'_> {
    match pair.as_str() {
        "_" => Environment::All,
        match_rule => Environment::Match(match_rule),
    }
}

fn build_input_output(pair: pest::iterators::Pair<'_, Rule>) -> &'_ str {
    pair.as_str()
}

fn build_sound(pair: pest::iterators::Pair<'_, Rule>) -> &'_ str {
    pair.as_str()
}

fn build_syllables(pair: pest::iterators::Pair<'_, Rule>) -> Vec<Vec<&'_ str>> {
    pair.into_inner().map(build_words).collect()
}

fn build_words(pair: pest::iterators::Pair<'_, Rule>) -> Vec<&'_ str> {
    pair.into_inner().map(|p| p.as_str()).collect()
}

fn build_class(pair: pest::iterators::Pair<'_, Rule>) -> (&'_ str, Vec<&'_ str>) {
    let mut pairs = pair.into_inner();
    let class_name = pairs.next().unwrap().as_str();
    let letters: Vec<_> = pairs.map(|p| p.as_str()).collect();
    (class_name, letters)
}

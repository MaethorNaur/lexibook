use std::collections::HashMap;
use std::fmt;

use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "wgl.pest"]
struct WGLParser;

pub type Letter<'a> = (&'a str, f64);

#[derive(Debug, Default)]
pub struct AST<'a> {
    pub imports: Vec<&'a str>,
    pub letters: Vec<Letter<'a>>,
    pub classes: HashMap<&'a str, Vec<&'a str>>,
    pub syllables: Vec<Vec<&'a str>>,
    pub rules: Vec<TransformationRule<'a>>,
    pub phonemes: HashMap<&'a str, (&'a str, Condition<'a>)>,
}

#[derive(Debug, Clone)]
pub enum Condition<'a> {
    Always,
    Not(ConditionType<'a>),
    And(Box<Condition<'a>>, Box<Condition<'a>>),
    Or(Box<Condition<'a>>, Box<Condition<'a>>),
    Single(ConditionType<'a>),
}

#[derive(Debug, Clone)]
pub enum ConditionType<'a> {
    None,
    BeginningWord,
    EndWord,
    FollowedBy(&'a str),
    Between(&'a str, &'a str),
}

#[derive(Debug)]
pub enum TransformationRule<'a> {
    SoundRule {
        environment: Environment<'a>,
        input: &'a str,
        output: Option<&'a str>,
    },
    PhonemeRule {
        input: &'a str,
        output: Option<&'a str>,
    },
}

#[derive(Debug)]
pub enum Environment<'a> {
    All,
    Match(&'a str),
}

impl<'a> TransformationRule<'a> {
    pub fn environment(&self) -> Option<&Environment<'a>> {
        match self {
            TransformationRule::SoundRule { environment, .. } => Some(&environment),
            TransformationRule::PhonemeRule { .. } => None,
        }
    }

    pub fn input(&self) -> &'a str {
        match self {
            TransformationRule::SoundRule { input, .. } => input,
            TransformationRule::PhonemeRule { input, .. } => input,
        }
    }

    pub fn output(&self) -> Option<&'a str> {
        match self {
            TransformationRule::SoundRule { output, .. } => *output,
            TransformationRule::PhonemeRule { output, .. } => *output,
        }
    }
}

impl<'a> fmt::Display for TransformationRule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode = match self {
            TransformationRule::SoundRule { .. } => "->",
            TransformationRule::PhonemeRule { .. } => "~>",
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

pub fn from_string(input: &'_ str) -> Result<AST, Error<Rule>> {
    let pairs = WGLParser::parse(Rule::wgl, &input)?;
    trace!("WGL parsed AST: {:#?}", pairs);
    let mut ast: AST = Default::default();
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
            Rule::phonemes => ast.phonemes = build_phonemes(pair),
            _ => {}
        }
    }

    trace!("AST: {:#?}", ast);

    Ok(ast)
}

fn build_phonemes(
    pair: pest::iterators::Pair<'_, Rule>,
) -> HashMap<&'_ str, (&'_ str, Condition<'_>)> {
    pair.into_inner()
        .map(|phoneme_pair| {
            let mut pair = phoneme_pair.into_inner();
            let letter = pair.next().unwrap().as_str();
            let notation = pair.next().unwrap().as_str();
            let condition = pair
                .next()
                .map(build_condition)
                .unwrap_or(Condition::Always);
            (letter, (notation, condition))
        })
        .collect()
}

fn build_condition_type(pair: pest::iterators::Pair<'_, Rule>) -> ConditionType<'_> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::beginning_word => ConditionType::BeginningWord,
        Rule::end_word => ConditionType::EndWord,
        Rule::between => {
            let mut rule = inner.into_inner();
            ConditionType::Between(rule.next().unwrap().as_str(), rule.next().unwrap().as_str())
        }
        _ => {
            let rule = inner.into_inner().next().unwrap();
            ConditionType::FollowedBy(rule.as_str())
        }
    }
}

fn build_condition(pair: pest::iterators::Pair<'_, Rule>) -> Condition<'_> {
    pair.into_inner()
        .fold(Condition::Always, |condition, inner| {
            let rule = inner.as_rule();
            match rule {
                Rule::NOT => Condition::Not(ConditionType::None),
                Rule::phoneme_condition => {
                    let condition_type = build_condition_type(inner);
                    match condition {
                        Condition::Not(_) => Condition::Not(condition_type),
                        _ => Condition::Single(condition_type),
                    }
                }
                Rule::AND => Condition::And(Box::new(condition), Box::new(Condition::Always)),
                Rule::OR => Condition::Or(Box::new(condition), Box::new(Condition::Always)),
                Rule::phoneme_conditions => {
                    let right = Box::from(build_condition(inner));
                    match condition {
                        Condition::And(left, _) => Condition::And(left, right),
                        Condition::Or(left, _) => Condition::Or(left, right),
                        _ => condition,
                    }
                }
                _ => condition,
            }
        })
}

fn build_imports(pair: pest::iterators::Pair<'_, Rule>) -> Vec<&'_ str> {
    pair.into_inner().map(|r| r.as_str()).collect()
}

#[allow(irrefutable_let_patterns)]
fn build_letters(pair: pest::iterators::Pair<'_, Rule>) -> Vec<Letter<'_>> {
    pair.into_inner()
        .map(|pair| {
            let mut inner = pair.into_inner();
            (
                inner.next().unwrap().as_str(),
                inner
                    .next()
                    .and_then(|p| p.as_str().parse::<f64>().ok())
                    .unwrap_or(0.0),
            )
        })
        .collect()
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
        Rule::sound_rule => Some(build_sound_rule(rule)),
        Rule::phoneme_rule => Some(build_phoneme_rule(rule)),
        _ => None,
    }
}

fn build_phoneme_rule(pair: pest::iterators::Pair<'_, Rule>) -> TransformationRule {
    let mut pairs = pair.into_inner();
    let input = pairs.next().map(build_input_output).unwrap();
    let sounds = pairs.as_str();
    let output = if sounds.is_empty() {
        None
    } else {
        Some(sounds)
    };

    TransformationRule::PhonemeRule { input, output }
}

fn build_sound_rule(pair: pest::iterators::Pair<'_, Rule>) -> TransformationRule<'_> {
    let mut pairs = pair.into_inner();
    let environment = pairs.next().map(build_environment).unwrap();
    let input = pairs.next().map(build_input_output).unwrap();
    let output = pairs.next().map(build_input_output);
    TransformationRule::SoundRule {
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

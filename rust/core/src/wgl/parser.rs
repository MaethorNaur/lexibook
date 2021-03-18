use pest::error::Error;
use pest::Parser;
use std::collections::HashMap;
use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(Parser)]
#[grammar = "wgl.pest"]
struct WGLParser;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Ast {
    pub imports: Vec<String>,
    pub letters: Vec<Letter>,
    pub classes: HashMap<String, Vec<String>>,
    pub syllables: Vec<Vec<String>>,
    pub rules: Vec<TransformationRule>,
    pub phonemes: HashMap<String, Vec<Phoneme>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Letter {
    pub letter: String,
    pub frequency: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Phoneme {
    pub notation: String,
    pub condition: Condition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Always,
    Not(ConditionType),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Single(ConditionType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    None,
    BeginningWord,
    EndWord,
    FollowedBy(String),
    Between(String, String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransformationRule {
    SoundRule {
        environment: Environment,
        input: String,
        output: Option<String>,
    },
    PhonemeRule {
        input: String,
        output: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Environment {
    All,
    Match(String),
}

impl Ast {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

pub fn from_json(input: &'_ str) -> Result<Ast, serde_json::Error> {
    serde_json::from_str(input)
}

impl TransformationRule {
    pub fn environment(&self) -> Option<&Environment> {
        match self {
            TransformationRule::SoundRule { environment, .. } => Some(&environment),
            TransformationRule::PhonemeRule { .. } => None,
        }
    }

    pub fn input(&self) -> String {
        match self {
            TransformationRule::SoundRule { input, .. } => input.to_string(),
            TransformationRule::PhonemeRule { input, .. } => input.to_string(),
        }
    }

    pub fn output(&self) -> Option<String> {
        match self {
            TransformationRule::SoundRule { output, .. } => output.clone(),
            TransformationRule::PhonemeRule { output, .. } => output.clone(),
        }
    }
}

impl fmt::Display for TransformationRule {
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
            self.output().unwrap_or_else(|| "".to_string())
        )
    }
}

impl Environment {
    pub fn to_string(&self) -> String {
        match self {
            Environment::All => "_".to_string(),
            Environment::Match(s) => s.to_string(),
        }
    }
}

pub fn from_string(input: &'_ str) -> Result<Ast, Error<Rule>> {
    let pairs = WGLParser::parse(Rule::wgl, &input)?;
    trace!("WGL parsed Ast: {:#?}", pairs);
    let mut ast: Ast = Default::default();
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
    trace!("ast: {:#?}", ast);

    Ok(ast)
}

fn build_phonemes(pair: pest::iterators::Pair<'_, Rule>) -> HashMap<String, Vec<Phoneme>> {
    let mut result = HashMap::new();
    pair.into_inner().for_each(|phoneme_pair| {
        let mut pair = phoneme_pair.into_inner();
        let letter = pair.next().unwrap().as_str().to_string();
        let notation = pair.next().unwrap().as_str().to_string();
        let condition = pair
            .next()
            .map(build_condition)
            .unwrap_or(Condition::Always);
        result.entry(letter).or_insert_with(Vec::new).push(Phoneme {
            notation,
            condition,
        });
    });
    result
}

fn build_condition_type(pair: pest::iterators::Pair<'_, Rule>) -> ConditionType {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::beginning_word => ConditionType::BeginningWord,
        Rule::end_word => ConditionType::EndWord,
        Rule::between => {
            let mut rule = inner.into_inner();
            ConditionType::Between(
                rule.next().unwrap().as_str().to_string(),
                rule.next().unwrap().as_str().to_string(),
            )
        }
        _ => {
            let rule = inner.into_inner().next().unwrap();
            ConditionType::FollowedBy(rule.as_str().to_string())
        }
    }
}

fn build_condition(pair: pest::iterators::Pair<'_, Rule>) -> Condition {
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

fn build_imports(pair: pest::iterators::Pair<'_, Rule>) -> Vec<String> {
    pair.into_inner().map(|r| r.as_str().to_string()).collect()
}

#[allow(irrefutable_let_patterns)]
fn build_letters(pair: pest::iterators::Pair<'_, Rule>) -> Vec<Letter> {
    pair.into_inner()
        .map(|pair| {
            let mut inner = pair.into_inner();
            Letter {
                letter: inner.next().unwrap().as_str().to_string(),
                frequency: inner
                    .next()
                    .and_then(|p| p.as_str().parse::<f64>().ok())
                    .unwrap_or(0.0),
            }
        })
        .collect()
}

fn build_rules(pair: pest::iterators::Pair<'_, Rule>) -> Vec<TransformationRule> {
    pair.into_inner()
        .filter_map(build_sound_or_phoneme_rule)
        .collect()
}

fn build_sound_or_phoneme_rule(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Option<TransformationRule> {
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
    let sounds = pairs.as_str().to_string();
    let output = if sounds.is_empty() {
        None
    } else {
        Some(sounds)
    };

    TransformationRule::PhonemeRule { input, output }
}

fn build_sound_rule(pair: pest::iterators::Pair<'_, Rule>) -> TransformationRule {
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

fn build_environment(pair: pest::iterators::Pair<'_, Rule>) -> Environment {
    match pair.as_str() {
        "_" => Environment::All,
        match_rule => Environment::Match(match_rule.to_string()),
    }
}

fn build_input_output(pair: pest::iterators::Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn build_syllables(pair: pest::iterators::Pair<'_, Rule>) -> Vec<Vec<String>> {
    pair.into_inner().map(build_words).collect()
}

fn build_words(pair: pest::iterators::Pair<'_, Rule>) -> Vec<String> {
    pair.into_inner().map(|p| p.as_str().to_string()).collect()
}

fn build_class(pair: pest::iterators::Pair<'_, Rule>) -> (String, Vec<String>) {
    let mut pairs = pair.into_inner();
    let class_name = pairs.next().unwrap().as_str().to_string();
    let letters: Vec<_> = pairs.map(|p| p.as_str().to_string()).collect();
    (class_name, letters)
}

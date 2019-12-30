use super::types::*;
use std::collections::HashMap;
use std::fs;

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
}

#[derive(Debug)]
pub enum Letter<'a> {
    WithPhoneticNotation(&'a str, Vec<&'a str>),
    OnlyRepresensation(&'a str),
}

impl<'a> Letter<'a> {
    pub fn to_string(&self) -> &'a str {
        match self {
            Letter::OnlyRepresensation(s) => s,
            Letter::WithPhoneticNotation(s, _) => s,
        }
    }
}

pub fn from_file(filename: &'_ str) -> Result<AST, Error<Rule>> {
    let input = Box::leak(
        fs::read_to_string(filename)
            .or_else(|e| Err(Error::IO(e)))?
            .into_boxed_str(),
    );
    from_string(input)
}

pub fn from_string(input: &'_ str) -> Result<AST, Error<Rule>> {
    let pairs = WGLParser::parse(Rule::wgl, &input).map_err(Error::Parse)?;
    debug!("{:#?}", pairs);
    let mut ast = AST {
        imports: vec![],
        letters: vec![],
        classes: HashMap::new(),
        syllables: vec![],
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
            _ => {}
        }
    }

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

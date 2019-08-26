#[derive(Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Comments,
    Words(Vec<Vec<Syllable<'a>>>),
    Class(&'a str, Vec<&'a str>),
    Letters(Vec<(&'a str, &'a str)>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Syllable<'a> {
    Optional,
    Class(char),
    Sound(&'a str),
}

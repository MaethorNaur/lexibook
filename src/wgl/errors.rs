use nom::types::CompleteStr;
use nom::Err;
use std::error;
use std::fmt;
use std::io;
#[derive(Debug)]
pub enum Error<'a> {
    UnknownLetter(&'a str, &'a str),
    IO(std::io::Error),
    Parse(Err<CompleteStr<'a>>),
}

impl From<io::Error> for Error<'_> {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}
impl<'a> From<(&'a str, &'a str)> for Error<'a> {
    fn from(letter_class: (&'a str, &'a str)) -> Self {
        Error::UnknownLetter(letter_class.0, letter_class.1)
    }
}
impl<'a> From<Err<CompleteStr<'a>>> for Error<'a> {
    fn from(err: Err<CompleteStr<'a>>) -> Self {
        Error::Parse(err)
    }
}

#[derive(Debug, Clone)]
pub struct UnknownIPACharacterError {
    value: char,
}
impl fmt::Display for UnknownIPACharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown IPA character: {}", self.value)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for UnknownIPACharacterError {
    fn description(&self) -> &str {
        "unknown IPA character: {}"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

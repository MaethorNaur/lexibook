use std::convert::From;
use std::error;
use std::fmt;
use std::io;
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Parse(pest::error::Error<lexibook::wgl::Rule>),
    Csv(csv::Error),
    Clap(clap::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
            Error::Csv(e) => e.fmt(f),
            Error::Clap(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<pest::error::Error<lexibook::wgl::Rule>> for Error {
    fn from(e: pest::error::Error<lexibook::wgl::Rule>) -> Self {
        Error::Parse(e)
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Error::Csv(e)
    }
}

impl From<clap::Error> for Error {
    fn from(e: clap::Error) -> Self {
        Error::Clap(e)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::IO(e) => e.description(),
            Error::Parse(e) => e.description(),
            Error::Csv(e) => e.description(),
            Error::Clap(e) => e.description(),
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IO(e) => e.source(),
            Error::Parse(e) => e.source(),
            Error::Csv(e) => e.source(),
            Error::Clap(e) => e.source(),
        }
    }
}

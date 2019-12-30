use pest::error;

pub enum Error<R> {
    IO(std::io::Error),
    Parse(error::Error<R>),
}

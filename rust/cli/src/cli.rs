use lexibook::sound_system::MonoSyllableRepartition;
use std::path::PathBuf;
use structopt::StructOpt;

#[allow(dead_code)]
fn variants() -> [&'static str; 6] {
    [
        "always",
        "mostly",
        "frequent",
        "less_frequent",
        "rare",
        "never",
    ]
}

#[derive(StructOpt)]
#[structopt(
    name = "lexibook",
    about = "Generate words and apply sound transformations"
)]
pub enum Cli {
    /// Display phonology
    Phonology(Phonology),
    /// Generate words and apply sound transformations
    Words(Words),
    /// Apply sound transformation on words
    Sounds(Sounds),
}
#[derive(StructOpt)]
pub struct Phonology {
    #[structopt(flatten)]
    pub common: Common,
    /// Word generation file definition
    pub filename: PathBuf,
}

#[derive(StructOpt)]
pub struct Words {
    #[structopt(flatten)]
    pub common: Common,
    /// Numbers of words to generate
    #[structopt(short, long, default_value = "10")]
    pub numbers: usize,
    /// Repartition of mono syllable words. Default to "less_frequent"
    #[structopt(short, long, default_value="less_frequent" , possible_values= &variants(), case_insensitive = true)]
    pub repartition: MonoSyllableRepartition,
    /// Not apply sound transformations
    #[structopt(long = "no-sound-transformations")]
    pub skip_transformation: bool,
    /// Word generation file definition
    pub filename: PathBuf,
}

#[derive(StructOpt)]
pub struct Sounds {
    #[structopt(flatten)]
    pub common: Common,
    /// Word generation file definition
    pub filename: PathBuf,
    /// Words
    pub input: Option<PathBuf>,
}

#[derive(StructOpt)]
pub struct Common {
    #[structopt(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
    /// File where the words will be outputed
    #[structopt(short, long)]
    pub output: Option<PathBuf>,
    /// Display pretty output
    #[structopt(short, long)]
    pub pretty: bool,
}

impl Cli {
    pub fn verbosity(&self) -> Option<log::Level> {
        let verbose = match self {
            Cli::Phonology(c) => &c.common.verbose,
            Cli::Sounds(c) => &c.common.verbose,
            Cli::Words(c) => &c.common.verbose,
        };
        verbose.log_level()
    }
}

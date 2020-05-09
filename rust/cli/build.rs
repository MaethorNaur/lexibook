use std::env;
use structopt::clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => PathBuf::from(outdir).join("..").join("..").join(".."),
    };
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, &outdir);
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, &outdir);
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::PowerShell, &outdir);
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Fish, &outdir);
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Elvish, &outdir);
}

#![allow(dead_code)]
use structopt::clap::Shell;
use structopt::StructOpt;
mod format {
    include!("src/format.rs");
}
mod bars {
    include!("src/bars.rs");
}
mod data {
    include!("src/data/mod.rs");
}
mod theme {
    include!("src/theme.rs");
}
mod cli {
    include!("src/cli.rs");
}
fn main() {
    // let name = env!("CARGO_BIN_NAME");
    let name = "macchina";
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    cli::Opt::clap().gen_completions(name, Shell::Fish, &outdir);
    cli::Opt::clap().gen_completions(name, Shell::Bash, &outdir);
    // cli::Opt::clap().gen_completions(name, Shell::Zsh, &outdir);
}

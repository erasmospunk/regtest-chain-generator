use clap::Clap;
use log::info;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(about, version)]
struct Opts {
    /// Path to a bitcoind executable
    #[clap(short, long, default_value = "bitcoind")]
    executable: PathBuf,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Generate(Generate),
}

/// Generate a synthetic blockchain
#[derive(Clap)]
struct Generate {
    /// Output directory for the generated chain
    #[clap(long, default_value = "chaingen_out")]
    output: PathBuf,
}

fn main() {
    simple_logger::init().unwrap();

    let opts: Opts = Opts::parse();

    // // Gets a value for config if supplied by user, or defaults to "default.conf"
    // println!("Value for config: {}", opts.config);
    info!("Using daemon `{}`", opts.executable.to_string_lossy());

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Generate(g) => info!("Output directory: {}", g.output.to_string_lossy()),
    }
}

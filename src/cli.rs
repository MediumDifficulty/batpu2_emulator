use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input file
    #[arg(short, long)]
    pub input: PathBuf,

    /// Runs the program in benchmark mode
    #[arg(short, long)]
    pub benchmark: bool,

    /// Runs the program without a GUI
    #[arg(short, long)]
    pub no_gui: bool,
}
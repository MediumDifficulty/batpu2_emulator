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

    /// Number of iterations to run the program
    #[arg(long, default_value_t = 1)]
    pub iterations: usize
}
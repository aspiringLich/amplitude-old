#![feature(let_chains)]
#![feature(once_cell)]

use std::sync::LazyLock;
use clap::Parser;

pub mod template_builder;

pub static ARGS: LazyLock<Args> = LazyLock::new(|| Args::parse());

#[derive(Parser, Debug)]
pub struct Args {
    /// Watch the input directory for changes
    #[arg(short, long)]
    pub watch: bool,
    /// The input directory to parse files from
    #[arg(short, long, default_value = "courses")]
    pub input: String,
    /// The output directory to output parsed files
    #[arg(short, long, default_value = "rendered")]
    pub output: String,
}

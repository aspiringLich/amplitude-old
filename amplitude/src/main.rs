use amplitude_markdown::parse::{parse_dir, parse_dir_watch};
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// Watch the input directory for changes
    #[arg(short, long)]
    watch: bool,
    /// The input directory to parse files from
    #[arg(short, long, default_value = "courses")]
    input: String,
    /// The output directory to output parsed files
    #[arg(short, long, default_value = "rendered")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();

    parse_dir(&args.input, &args.output)?;
    if args.watch {
        let input = args.input.clone();
        let output = args.output.clone();
        parse_dir(&input, &output)?;
        std::thread::spawn(|| parse_dir_watch(input, output));
    }

    Ok(())
}

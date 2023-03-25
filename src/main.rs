use amplitude_markdown::parse::{parse_dir, parse_dir_watch};
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// Whether to watch the input directory for changes
    #[arg(short, long)]
    watch: bool,
    /// The input directory to parse files from
    #[arg(short, long, default_value = "media/input")]
    input: String,
    /// The output directory to parse files from
    #[arg(short, long, default_value = "media/output")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();

    match args.watch {
        true => {
            let input = args.input.clone();
            let output = args.output.clone();
            std::thread::spawn(move || parse_dir_watch(input, output));
        }
        false => parse_dir(&args.input, &args.output)?,
    }

    Ok(())
}

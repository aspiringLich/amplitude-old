use amplitude_common::ARGS;
use amplitude_markdown::parse::{parse_dir, parse_dir_watch};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    parse_dir(&ARGS.input, &ARGS.output)?;
    if ARGS.watch {
        std::thread::spawn(|| parse_dir_watch());
    }

    Ok(())
}

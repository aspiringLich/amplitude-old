use amplitude_common::config;
use amplitude_markdown::parse::parse_dir;

fn main() {
    parse_dir(&config::INPUT, &config::RENDERED).unwrap();
}

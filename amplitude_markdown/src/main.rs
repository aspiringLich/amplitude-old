use amplitude_common::path;
use amplitude_markdown::parse::parse_dir;

fn main() {
    parse_dir(&path::INPUT, &path::RENDERED).unwrap();
}

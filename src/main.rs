use std::path::Path;

use amplitude_markdown::parse::parse_dir;

fn main() {
    parse_dir(Path::new("./media/input"), Path::new("./media/output")).unwrap();
}

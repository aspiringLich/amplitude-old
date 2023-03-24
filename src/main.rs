use std::path::Path;

use parser::{parse::parse_dir, link_concat::LinkMap};

fn main() {
    parse_dir(Path::new("./media/input"), Path::new("./media/output"), LinkMap::new());
}

use std::path::Path;

use parser::{link_concat::LinkMap, parse::parse_dir};

fn main() {
    parse_dir(
        Path::new("./media/input"),
        Path::new("./media/output"),
    )
    .unwrap();
}

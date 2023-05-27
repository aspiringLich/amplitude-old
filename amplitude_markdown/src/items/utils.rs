use std::{collections::HashSet, path::Path};

use amplitude_common::lang::Language;
use enum_iterator::all;

pub enum DirItem {
    Code { name: String, ext: String },
    Markdown { name: String },
    Toml { name: String },
    Dir { name: String },
    Other { name: String, ext: String },
}

fn os_to_str(s: &std::ffi::OsStr) -> String {
    s.to_str().unwrap().to_string()
}

pub fn get_dir_contents(path: &Path) -> Vec<DirItem> {
    let mut items = Vec::new();
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let name = os_to_str(path.file_stem().unwrap());
        let ext = os_to_str(path.extension().unwrap_or_default());

        let code_ext = all::<Language>()
            .map(|l| l.extension())
            .collect::<HashSet<_>>();

        let item = match ext.as_str() {
            "md" => DirItem::Markdown { name },
            "toml" => DirItem::Toml { name },
            s @ _ if code_ext.contains(s) => DirItem::Code { name, ext },
            _ => DirItem::Other { name, ext },
        };
        items.push(item);
    }
    items
}

use std::path::{Component, Path};

pub fn get_path_components(path: &Path) -> impl Iterator<Item = String> + '_ {
    path.components().filter_map(|c| {
        matches!(c, Component::Normal(_)).then(|| c.as_os_str().to_str().unwrap().to_string())
    })
}

use std::{env, path::PathBuf};

use amplitude_runner::rebuild_images;

fn main() {
    let binding = env::current_dir().unwrap().to_path_buf();
    let dir = binding
        .components()
        .take_while(|c| c.as_os_str() != "amplitude")
        .collect::<PathBuf>()
        .join("amplitude");
    env::set_current_dir(&dir).unwrap();
    rebuild_images()
}

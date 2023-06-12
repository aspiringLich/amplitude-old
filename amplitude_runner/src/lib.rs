#![feature(default_free_fn)]
#![feature(iter_intersperse)]

pub mod lang;
pub mod var_type;
pub mod runner;
pub mod exercise;

use amplitude_common::{config::LanguageConfig, path};
use std::{
    collections::HashMap,
    env, fs,
    path::Path,
    process::{self, Command},
};

pub fn rebuild_images() {
    if env::current_dir().unwrap().file_name().unwrap() == "amplitude_runner" {
        env::set_current_dir("../").unwrap();
    }
    env::set_current_dir(&path::LANGUAGES).unwrap();

    let base_dir = env::current_dir().unwrap();
    let langs = load_langs("languages.toml");

    for i in langs {
        env::set_current_dir(base_dir.join(i.1)).unwrap();

        let run = Command::new("docker")
            .args(["build", "-t", &i.2, "."])
            .status()
            .unwrap();

        if !run.success() {
            process::exit(-1);
        }
    }
}

fn load_langs<T: AsRef<Path>>(file: T) -> Vec<(String, String, String)> {
    let langs: HashMap<String, LanguageConfig> = toml::from_str(&fs::read_to_string(file).unwrap())
        .expect("Error parsing languages/languages.toml");
    let mut out = Vec::new();

    for (path, lang) in langs {
        out.push((path.clone(), path, lang.image_name));
    }

    out
}

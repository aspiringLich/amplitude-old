#![feature(iter_intersperse)]

pub mod exercise;
pub mod lang;
pub mod runner;
pub mod var_type;

use amplitude_common::{
    config::LanguageConfig,
    path::{self, scope_dir},
};
use std::{
    collections::HashMap,
    env, fs,
    process::{self, Command},
};

pub fn rebuild_images() {
    // maybe i can figure out whats goin on here
    dbg!(env::current_dir().unwrap());
    dbg!(fs::read_dir(".").unwrap());
    scope_dir(&path::LANGUAGES, || {
        let langs: HashMap<String, LanguageConfig> =
            toml::from_str(&fs::read_to_string("languages.toml").unwrap()).unwrap();

        for (lang, cfg) in langs {
            scope_dir(lang, || {
                let run = Command::new("docker")
                    .args(["build", "-t", &cfg.image_name, "."])
                    .status()
                    .unwrap();

                if !run.success() {
                    process::exit(-1);
                }
            })
            .unwrap();
        }
    })
    .unwrap();
}

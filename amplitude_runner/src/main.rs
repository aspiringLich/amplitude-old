// cargo-deps: serde_json = "1.0.79"

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};

use serde::Deserialize;

fn main() {
    if env::current_dir().unwrap().file_name().unwrap() == "amplitude_runner" {
        env::set_current_dir("../").unwrap();
    }
    env::set_current_dir("langs").unwrap();

    let base_dir = env::current_dir().unwrap();
    let langs = load_langs("languages.toml");

    for i in langs {
        println!("[BUILDNIG] {}", i.0);
        env::set_current_dir(base_dir.join(i.1)).unwrap();

        let run = Command::new("docker")
            .args(["build", "-t", &i.2, "."])
            .status()
            .unwrap();

        if !run.success() {
            println!("[ERROR] exiting");
            process::exit(-1);
        }
    }
}


fn load_langs<T: AsRef<Path>>(file: T) -> Vec<(String, String, String)> {
    #[derive(Deserialize)]
    struct Lang {
        image_name: String,
        // source_path: String,
    }

    let langs: HashMap<String, Lang> =
        toml::from_str(&fs::read_to_string(file).unwrap()).expect("Error parsing langs/languages.toml");
    let mut out = Vec::new();

    for (path, lang) in langs {
        out.push((
            path.clone(),
            path,
            lang.image_name,
        ));
    }

    out
}

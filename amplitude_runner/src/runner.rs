use std::{
    collections::HashMap,
    fs,
    io::BufRead,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use amplitude_common::config::{DockerConfig, LanguageConfig};
use anyhow::Context;
use serde::{Deserialize, Serialize};

pub fn url_encode(url: &str) -> String {
    const ALLOWED_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                   abcdefghijklmnopqrstuvwxyz\
                                   0123456789-._~";

    let mut out = String::with_capacity(url.len());

    for i in url.chars() {
        if i.is_ascii() && ALLOWED_CHARS.contains(&(i as u8)) {
            out.push(i);
            continue;
        }
        out.push_str(&format!("%{:02X}", i as u8));
    }

    out
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
    pub runtime: Duration,
    pub exit_code: i32,
}

pub fn run(
    lang: &LanguageConfig,
    cfg: &DockerConfig,
    src: &str,
    other_files: HashMap<String, &[u8]>,
    args: &str,
) -> anyhow::Result<RunOutput> {
    // test that the docker image exists
    #[cfg(debug_assertions)]
    {
        let out = Command::new(&cfg.command)
            .arg("images")
            .arg(&lang.image_name)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .context("While running `docker images`")?;
        if out
            .stdout
            .lines()
            .skip(1)
            .filter_map(|x| x.ok())
            .all(|x| !x.starts_with(&lang.image_name))
        {
            dbg!(
                "Image {} not found! try running `cd amplitude_runner` and `cargo r` to rebuild \
                 docker images",
                &lang.image_name
            );
            std::process::exit(-1);
        }
    }

    let tempdir = tempfile::tempdir_in(&cfg.tmp_folder).context("While creating temp dir")?;
    let code_path = tempdir.path().join(&lang.source_path);
    fs::create_dir_all(code_path.parent().unwrap()).context("While creating temp dir")?;
    fs::write(&code_path, src).context("While writing source code")?;

    for (path, content) in &other_files {
        let path = tempdir.path().join(path);
        fs::create_dir_all(path.parent().unwrap()).context("While creating temp dir")?;
        fs::write(&path, content).context("While writing file")?;
    }

    let time = Instant::now();

    let v = [&lang.source_path]
        .into_iter()
        .chain(other_files.keys())
        .map(|path| {
            format!(
                "{}:/runner/{}",
                tempdir.path().join(path).to_string_lossy(),
                &path
            )
        })
        .collect::<Vec<_>>();

    // tried to use bollard instead of using a command but that was even worse
    let run = Command::new(&cfg.command)
        .args(
            [
                "run",
                "--rm",
                "--cap-drop=ALL",
                "--security-opt=no-new-privileges",
                "--net",
                "none",
                "--memory",
                "128m",
                "--memory-swap",
                "128m",
                "--pids-limit",
                "512",
                "-e",
                &format!("TIMEOUT={}", &cfg.timeout),
                "-e",
                &format!("ARGS={}", url_encode(args)),
                "-v",
            ]
            .into_iter()
            .chain(v.iter().map(|x| x.as_str()).intersperse("-v"))
            .chain([lang.image_name.as_str()]),
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    Ok(RunOutput {
        stdout: String::from_utf8_lossy(&run.stdout).to_string(),
        stderr: String::from_utf8_lossy(&run.stderr).to_string(),
        runtime: time.elapsed(),
        exit_code: run.status.code().unwrap(),
    })
}

#[cfg(test)]
mod test {
    use std::{env, fs};

    use amplitude_common::config::{Args, Config};

    use super::*;

    #[test]
    fn test_runner() -> anyhow::Result<()> {
        env::set_current_dir("../")?;
        let args = Args::parse();
        let mut config: Config = toml::from_str::<Config>(&fs::read_to_string(&args.config)?)?;
        config.args = args;

        let output = run(
            config
                .docker
                .language_config
                .get("python")
                .expect("Python not found"),
            &config.docker,
            "
print('Hello, World!')

file = open('file.txt', 'r')
print(file.read())
file.close()
",
            HashMap::from_iter([("file.txt".to_string(), "File contents".as_bytes())]),
            "",
        )
        .unwrap();
        dbg!(&output);
        assert!(output.stdout == "Hello, World!\nFile contents\n");

        Ok(())
    }
}

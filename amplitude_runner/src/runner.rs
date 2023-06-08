use std::{
    io::Write,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use amplitude_common::config::{Docker, LanguageConfig};
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

pub fn run(lang: &LanguageConfig, cfg: &Docker, src: &str, args: &str) -> anyhow::Result<RunOutput> {
    let mut code_file = tempfile::NamedTempFile::new_in(&cfg.tmp_folder).unwrap();
    code_file.write_all(src.as_bytes()).unwrap();

    let time = Instant::now();
    let run = Command::new(&cfg.command)
        .args([
            "run",
            "--rm",
            "--cap-drop=ALL",
            "--security-opt=no-new-privileges",
            "--net",
            "none",
            "--memory",
            "128m",
            "--memory-swap",
            "256m",
            "--pids-limit",
            "512",
            "-v",
            &format!(
                "{}:/runner/{}",
                code_file.path().to_str().unwrap(),
                lang.source_path
            ),
            "-e",
            &format!("TIMEOUT={}", &cfg.timeout),
            "-e",
            &format!("ARGS={}", url_encode(args)),
            &lang.image_name,
        ])
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
    use std::env;

    use amplitude_common::config::{Args, Config};

    use super::*;

    #[test]
    fn test_runner() -> anyhow::Result<()> {
        env::set_current_dir("../")?;
        let args = Args::parse();
        let mut config: Config =
            toml::from_str::<Config>(&fs::read_to_string( &args.config)?)?;
        config.args = args;

        let output = run(
            &config.docker.language_config.get("python").expect("Python not found"),
            &config.docker,
            "print('Hello, World!')",
            "",
        )?;
        dbg!(&output);
        assert!(output.stdout == "Hello, World!\n");

        Ok(())
    }
}

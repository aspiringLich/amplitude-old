use std::{
    default::default,
    // default::default,
    io::{self, Write},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use amplitude_common::config::{DockerConfig, LanguageConfig};
use anyhow::Context;
use bollard::{
    container::{self, ListContainersOptions, WaitContainerOptions, AttachContainerOptions, StartContainerOptions, UpdateContainerOptions, CreateContainerOptions, AttachContainerResults},
    service::{ContainerCreateResponse, RestartPolicy},
    Docker,
};
use futures::StreamExt;
// use anyhow::Context;
// use bollard::{container::StartContainerOptions, Docker};
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

async fn docker(
    lang: &LanguageConfig,
    cfg: &DockerConfig,
) -> anyhow::Result<(
    ContainerCreateResponse,
    Docker,
    AttachContainerResults,
)> {
    let docker =
        Docker::connect_with_local_defaults().context("while attempting to connect to docker")?;

    let create = docker
        .create_container(
            None::<CreateContainerOptions<&str>>,
            container::Config {
                image: Some(lang.image_name.clone()),
                env: Some(vec![format!("TIMEOUT={}", &cfg.timeout)]),
                stop_timeout: Some(cfg.timeout as i64),
                
                // cmd: Some(vec!["tail", "-f", "/dev/null"]),
                ..default()
            },
        )
        .await
        .context("While creating container")?;

    docker
        .update_container::<&str>(
            &lang.image_name,
            UpdateContainerOptions {
                memory: Some(128 * 1024 * 1024),
                memory_swap: Some(128 * 1024 * 1024),
                pids_limit: Some(512),
                restart_policy: Some(RestartPolicy {
                    name: Some(bollard::service::RestartPolicyNameEnum::NO),
                    maximum_retry_count: None,
                }),
                ..default()
            },
        )
        .await
        .context("While updating container")?;

    let attach = docker
        .attach_container(
            &lang.image_name,
            Some(AttachContainerOptions::<&str> {
                stream: Some(true),
                stdout: Some(true),
                stderr: Some(true),
                logs: Some(true),
                ..default()
            }),
        )
        .await
        .context("While attaching container")?;

    docker
        .start_container(
            &lang.image_name,
            None::<StartContainerOptions<&str>>,
        )
        .await
        .context("While starting container")?;

    Ok((create, docker, attach))
}

pub async fn run(
    lang: &LanguageConfig,
    cfg: &DockerConfig,
    src: &str,
    args: &str,
) -> anyhow::Result<RunOutput> {
    let mut code_file =
        tempfile::NamedTempFile::new_in(&cfg.tmp_folder).context("While creating temp dir")?;
    code_file
        .write_all(src.as_bytes())
        .context("While writing code to file")?;

    let time = Instant::now();

    let (create, docker, mut attach) = docker(lang, cfg)
        .await
        .context("While initializing container")?;
    // panic!("{:#?}", &create);

    let mut stdout = io::stdout().lock();

    // pipe docker attach output into stdout
    while let Some(Ok(output)) = attach.output.next().await {
        stdout.write_all(output.into_bytes().as_ref())?;

    }

    // for _ in 0..2 {
    //     dbg!(
    //         docker
    //             .list_containers(None::<ListContainersOptions<&str>>)
    //             .await
    //     );
    //     tokio::time::sleep(Duration::from_secs_f32(1.0)).await;
    // }
    
    let mut stream = docker.wait_container(
        &lang.image_name,
        None::<WaitContainerOptions<&str>>,
    );
    // panic!();

    dbg!(stream.next().await);
    todo!()

    // Ok(RunOutput {
    //     stdout: String::from_utf8_lossy(&run.stdout).to_string(),
    //     stderr: String::from_utf8_lossy(&run.stderr).to_string(),
    //     runtime: time.elapsed(),
    //     exit_code: run.status.code().unwrap(),
    // })

    // futures::executor::block_on(docker.start_container(
    //     &lang.image_name,
    //     None::<StartContainerOptions<&'static str>>,
    // )).context("While starting container")?;
    // let run = Command::new(&cfg.command)
    //     .args([
    //         "run",
    //         "--rm",
    //         "--cap-drop=ALL",
    //         "--security-opt=no-new-privileges",
    //         "--net",
    //         "none",
    //         "--memory",
    //         "128m",
    //         "--memory-swap",
    //         "256m",
    //         "--pids-limit",
    //         "512",
    //         "-v",
    //         &format!(
    //             "{}:/runner/{}",
    //             code_file.path().to_str().unwrap(),
    //             lang.source_path
    //         ),
    //         "-e",
    //         &format!("TIMEOUT={}", &cfg.timeout),
    //         "-e",
    //         &format!("ARGS={}", url_encode(args)),
    //         &lang.image_name,
    //     ])
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .spawn()
    //     .unwrap()
    //     .wait_with_output()
    //     .unwrap();
}

#[cfg(test)]
mod test {
    use std::{env, fs};

    use amplitude_common::config::{Args, Config};

    use super::*;

    #[tokio::test]
    async fn test_runner() -> anyhow::Result<()> {
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
            "print('Hello, World!')",
            "",
        )
        .await
        .unwrap();
        dbg!(&output);
        assert!(output.stdout == "Hello, World!\n");

        Ok(())
    }
}

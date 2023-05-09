use std::{fs, path::Path};

use amplitude_common::config::{Config, ParseConfig};
use anyhow::{ensure, Context};

use git2::build::RepoBuilder;

use tracing::{info, warn};

use crate::{
    inject::{self},
    link_concat::link_concat_callback,
    state::{
        article::{parse_frontmatter, ArticleConfig},
        ParseState,
    },
};
use comrak::{parse_document_refs, Arena, ComrakOptions, RefMap};

/// Reparses the things and does the things
pub fn parse(config: &Config) -> anyhow::Result<ParseState> {
    if !config.args.local {
        info!("Deleting `{}` and recloning repo... (If you dont want this behavior, run with `--local`)", config.parse.clone_path);
        clone_repo(&config.parse).context("While cloning repo")?;
    } else {
        info!(
            "Using local files in `{}` for articles",
            config.parse.clone_path
        );
    }

    info!("Parsing articles...");
    let mut state = parse_dir(&config.parse).context("")?;
    
    let index_path = config.parse.clone_path.clone() + "/index.toml";
    let index = fs::read_to_string(&index_path)
        .with_context(|| format!("While reading {}", index_path))?;
    state.courses = toml::from_str(&index)?;
    
    dbg!(&state);

    Ok(state)
}

/// Parse the input `md` and return the output `html`.
pub(crate) fn parse_md(
    config: &ArticleConfig,
    input: &str,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<(String, RefMap)> {
    // get the refs
    let mut this_refs = parse_document_refs(&Arena::new(), input);
    if !this_refs.map.is_empty() {
        this_refs.extend(refs.clone());
    } else {
        this_refs = refs.clone();
    }

    // were not modifying options, so we can be sneaky
    // also im just too lazy to refactor this
    let options = unsafe { &*(&state.options as *const ComrakOptions) };

    let arena = Arena::new();
    let out = comrak::parse_document_with_broken_link_callback(
        &arena,
        input,
        options,
        Some(&mut |link| {
            let out = link_concat_callback(link, &this_refs);
            if out.is_none() {
                warn!("Broken link `{}` in {:?}", link, config.id);
            }
            out
        }),
    );
    // do things
    inject::inject(config, out, &this_refs, state)?;

    let mut cm = vec![];
    comrak::format_html(out, options, &mut cm).context("while parsing AST to html")?;

    Ok((String::from_utf8(cm).unwrap(), this_refs))
}

/// Clones the articles repo
pub fn clone_repo(config: &ParseConfig) -> anyhow::Result<()> {
    let clone_path = &config.clone_path;
    fs::create_dir_all(clone_path)?;

    // delete everything
    for item in fs::read_dir(clone_path)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    RepoBuilder::new().clone(&config.git_url, Path::new(clone_path))?;

    Ok(())
}

/// assuming `config.articles.clone_path` exists, parses the articles into html
pub fn parse_dir(config: &ParseConfig) -> anyhow::Result<ParseState> {
    let mut state = ParseState::default();
    let input = Path::new(&config.clone_path);
    let output = Path::new(&config.output_path);

    // get the top level header file
    let (_, refs) = parse_md(
        &ArticleConfig::default(),
        &fs::read_to_string(input.join("header.md"))?,
        &RefMap::new(),
        &mut state,
    )?;

    fs::create_dir_all(output)?;
    // delete everything in output
    for item in fs::read_dir(output)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    // recursively parse input directory
    for item in fs::read_dir(input)? {
        let item = item?;
        let path = item.path();

        if path.is_dir() {
            let name = path.file_name().unwrap();
            if name.to_str().unwrap().starts_with('.') {
                continue;
            }

            fs::create_dir(&output.join(name))?;
            let suffix = path.strip_prefix(input).unwrap();

            if path.join("header.md").exists() {
                let s = fs::read_to_string(&path.join("header.md"))?;
                let (_, refs) = parse_md(&ArticleConfig::default(), &s, &refs, &mut state)
                    .with_context(|| format!("While parsing header file {path:?}/header.md"))?;

                internal_parse_dir(config, &path, &output.join(suffix), &refs, &mut state)
                    .with_context(|| format!("While parsing dir {path:?}"))?;
            } else {
                internal_parse_dir(config, &path, &output.join(suffix), &refs, &mut state)
                    .with_context(|| format!("While parsing dir {path:?}"))?;
            }
        }
    }

    Ok(state)
}

fn internal_parse_dir<P: AsRef<Path>>(
    config: &ParseConfig,
    input: P,
    output: P,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<()> {
    let input = input.as_ref();
    let output = output.as_ref();

    for item in fs::read_dir(input)? {
        let item = item?;
        let path = item.path();

        if path.is_file() {
            let name = path.file_name().unwrap();
            match path.extension().map(|s| s.to_str().unwrap()) {
                Some("md") => {}
                _ => {
                    fs::copy(&path, output.join(name))?;
                }
            }
            if name == "header.md" {
                continue;
            }

            let (article_config, s) = parse_frontmatter(&path)?;
            let (html, _) = parse_md(&article_config, &s, refs, state)
                .with_context(|| format!("While parsing file {path:?}"))?;

            ensure!(
                !state.has_id(&article_config.id),
                "Duplicate article id: {}",
                article_config.id
            );
            let output = output.join(name).with_extension("html");
            state.insert_article(article_config, &output);
            fs::write(output, html)?;
        } else if path.is_dir() {
            let name = path.file_name().unwrap();

            // ignore hidden folders
            if name.to_str().unwrap().starts_with('.') {
                continue;
            }

            fs::create_dir(&output.join(name))?;
            let output = output.join(path.strip_prefix(input).unwrap());

            if input.join("header.md").exists() {
                let s = fs::read_to_string(&input.join("header.md"))?;
                let (_, refs) = parse_md(&ArticleConfig::default(), &s, refs, state)
                    .with_context(|| format!("While parsing header file {path:?}/header.md"))?;

                internal_parse_dir(config, &path, &output, &refs, state)
                    .with_context(|| format!("While parsing dir {path:?}"))?;
            } else {
                internal_parse_dir(config, &path, &output, refs, state)
                    .with_context(|| format!("While parsing dir {path:?}"))?;
            }
        }
    }

    Ok(())
}

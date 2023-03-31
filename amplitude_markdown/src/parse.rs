use std::{collections::HashMap, default::default, fs, path::Path, sync::{Arc, Mutex}};

use amplitude_common::config;
use anyhow::Context;
use notify::{Config, RecommendedWatcher, Watcher};
use tracing::{error, info};

use crate::{
    inject::{self, ArticleRef},
    link_concat::link_concat_callback,
};
use comrak::{
    parse_document_refs, Arena, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions,
    ListStyleType, RefMap,
};

#[derive(Debug)]
pub struct ParseState {
    pub questions: HashMap<(String, String, String), inject::quiz::Quiz>,
}

impl ParseState {
    pub fn get_question(&self, article: ArticleRef, question: &str) -> Option<&inject::quiz::Quiz> {
        self.questions.get(&(
            article.course.to_string(),
            article.article.to_string(),
            question.to_string(),
        ))
    }

    pub fn insert_question(
        &mut self,
        article: ArticleRef,
        question: &str,
        quiz: inject::quiz::Quiz,
    ) -> Option<inject::quiz::Quiz> {
        self.questions.insert(
            (
                article.course.to_string(),
                article.article.to_string(),
                question.to_string(),
            ),
            quiz,
        )
    }
}

/// Parse the input `md` and return the output `html`.
///
/// ## Behavior
///
/// - Link concatenation is supported
pub(crate) fn parse(
    article: ArticleRef,
    input: &str,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<String> {
    let mut this_refs = parse_document_refs(&Arena::new(), input);
    this_refs.extend(refs.clone());

    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            header_ids: None,
            footnotes: true,
            description_lists: true,
            front_matter_delimiter: Some("---".to_string()),
        },
        parse: default(),
        render: ComrakRenderOptions {
            github_pre_lang: true,
            full_info_string: true,
            unsafe_: true,
            hardbreaks: false,
            width: 0,
            escape: false,
            list_style: ListStyleType::default(),
        },
    };

    let arena = Arena::new();
    let out = comrak::parse_document_with_broken_link_callback(
        &arena,
        input,
        &options,
        Some(&mut |link| link_concat_callback(link, &this_refs)),
    );
    // do things
    inject::inject(article, out, refs, state)?;

    let mut cm = vec![];
    comrak::format_html(out, &options, &mut cm)
        .context("While parsing AST to html")?;

    Ok(String::from_utf8(cm).unwrap())
}

/// Parse all files in the input directory and all its subdirectories, and write
/// the output to the output directory. Any files included in the input will
/// also be copied over to the output directory. It will delete any files in the
/// output directory that have no counterpart in the input directory.
///
/// When encountering a file called `header.md`, it will parse it as a link
/// definition file and it will not be included in the output.
///
/// `header.md` files will add on to each other, and apply to all the files in
/// their directory and below it.
///
/// ## Notes on Behavior
///
///  - `.md` files will be parsed and converted to `.html` files
///  - `.md` files in the output directory will be removed
///  - `.toml` files in the output directory will be removed
///
/// ## Special Files
///
///  - `header.md` files will be parsed as link definition files and will not be
///    included in the output
///  - `config.toml` files will be parsed to register the course
///
pub fn parse_dir<P: AsRef<Path>>(input: P, output: P) -> anyhow::Result<ParseState> {
    if !output.as_ref().exists() {
        fs::create_dir_all(output.as_ref())?;
    }

    let mut state = ParseState {
        questions: HashMap::new(),
    };
    let article = ArticleRef {
        course: "",
        article: "",
    };

    if let Ok(s) = fs::read_to_string(input.as_ref().join("header.md")) {
        let refs = comrak::parse_document_refs(&Arena::new(), &s);
        parse_dir_internal(article, 0, input, output, &refs, &mut state)?;
    } else {
        parse_dir_internal(article, 0, input, output, &RefMap::new(), &mut state)?;
    }
    
    // dbg!(state);
    Ok(state)
}

fn parse_dir_internal<P: AsRef<Path>>(
    article: ArticleRef,
    depth: u8,
    input: P,
    output: P,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<()> {
    let input = input.as_ref();
    let output = output.as_ref();

    // check for files in output that are not in input, and delete them
    for entry in fs::read_dir(output)? {
        let o = entry?.path();
        let mut i = input.join(o.strip_prefix(output).unwrap());

        match o.extension().and_then(|e| e.to_str()).unwrap_or_default() {
            // check for the md counterpart
            "html" => {
                i.set_extension("md");
                // dbg!(&i, &o);
                if !i.exists() {
                    fs::remove_file(o)?;
                }
            }
            // just get rid of it
            "md" | "toml" => fs::remove_file(o)?,
            _ => {
                // dbg!(&o);
                if !i.exists() {
                    // dbg!(&i, &o);
                    fs::remove_file(o)?;
                }
            }
        }
    }

    // go through all the files, copying them over and parsing them
    for entry in fs::read_dir(input)? {
        let i = entry?.path();
        let o = output.join(i.clone().strip_prefix(input).unwrap());

        // if its a dir, update links and call the fn again
        if i.is_dir() {
            if !o.exists() {
                fs::create_dir(&o)?;
            }
            let mut a = article.clone();
            let name = i.file_name().unwrap().to_str().unwrap();
            match depth {
                0 => a.course = name,
                _ => {}
            }
            if let Ok(s) = fs::read_to_string(input.join("header.md")) {
                let other_refs = comrak::parse_document_refs(&Arena::new(), &s);
                let mut refs = refs.clone();
                refs.extend(other_refs);

                parse_dir_internal(a, depth + 1, &i, &o, &refs, state)?;
            } else {
                parse_dir_internal(a, depth + 1, &i, &o, refs, state)?;
            }
            continue;
        }

        let ext = i.extension().unwrap_or_default();
        let name = i.file_name().unwrap_or_default();

        match ext.to_str().unwrap_or_default() {
            "md" => {
                // ignore header.md
                if name == "header.md" {
                    continue;
                }

                // otherwise, parse
                let s = fs::read_to_string(&i)?;
                let i = i.display();
                anyhow::ensure!(
                    depth == 1,
                    "File: {name:?} must be in the article directory"
                );
                let mut article = article.clone();
                article.article = name.to_str().unwrap();
                let output =
                    parse(article, &s, refs, state).context(format!("While parsing file {i}"))?;
                fs::write(o.with_extension("html"), output)?;
            }
            "toml" if name == "config.toml" => {
                // // parse the config file
                // let config = parse_config(&i)?;

                // // generate the index file
                // let index = generate_index(&config, &links);
                // let path = o.with_file_name("index.html");
                // fs::write(path, index)?;
            }
            _ => {
                // copy over the file
                fs::copy(i, o)?;
            }
        }
    }

    Ok(())
}

/// This function will watch the input directory and write to the output
/// directory when detecting file changes using the `notify` crate.
///
/// See [`parse_dir`] for more description on how this function behaves
pub fn parse_dir_watch(state: Arc<Mutex<ParseState>>) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(config::INPUT.as_path(), notify::RecursiveMode::Recursive)?;

    info!("Watching for changes in '{}'", config::INPUT);

    while let Ok(mut event) = rx.recv() {
        use notify::EventKind::*;

        // wait 50ms to avoid duplicate events
        std::thread::sleep(std::time::Duration::from_millis(50));

        // drain the channel
        while let Ok(e) = rx.try_recv() {
            match e {
                Ok(e) if matches!(e.kind, Create(_) | Modify(_) | Remove(_)) => event = Ok(e),
                Err(e) => error!("Error watching directory: {:?}", e),
                _ => (),
            }
        }

        match event {
            Ok(event) if matches!(event.kind, Create(_) | Modify(_) | Remove(_)) => {
                info!("Change detected, reparsing...");
                match parse_dir(&config::INPUT, &config::OUTPUT) {
                    Err(e) => error!("Error parsing directory: '{:?}'", e),
                    Ok(s) => *state.lock().unwrap() = s,
                }
            }
            Err(e) => error!("Error watching directory: {:?}", e),
            _ => (),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {}

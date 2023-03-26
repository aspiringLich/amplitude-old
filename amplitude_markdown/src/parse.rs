use std::{
    borrow::Cow,
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use notify::{Config, Error, Event, RecommendedWatcher, Watcher};
use pulldown_cmark::{html, Options, Parser, RefDefs};
use tracing::{error, info};

use crate::link_concat::{get_links_of, link_concat_events, LinkDefs};

/// Parse the input text and return the output text.
pub(crate) fn parse(input: &str, links: &LinkDefs) -> String {
    // let input = fs::read_to_string(input)?;
    let other: LinkDefs;
    get_links_of!(input, other);
    let events = link_concat_events(&input, Options::all(), links, &other);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());

    html_output
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
/// their directory and below it. For example consider the following directory
/// structure:
///
/// ```txt
/// courses
/// ├── rust
/// │   ├── header.md
/// │   ├── config.toml
/// │   ├── types.md
/// │   ├── functions.md
/// │   ├── structs.md
/// │   └── enums.md
/// └── c
///     ├── header.md
///     ├── config.toml
///     ├── variables.md
///     ├── functions.md
///     ├── structs.md
///     └── enums.md
/// ```
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
///  - `config.toml` files will be parsed to register the course and generate
///    the `index.html` file
///
pub fn parse_dir<P: AsRef<Path>>(input: P, output: P) -> std::io::Result<()> {
    if let Ok(s) = fs::read_to_string(input.as_ref().join("header.md")) {
        let links: LinkDefs;
        get_links_of!(&s, links);
        parse_dir_internal(input, output, &links)
    } else {
        parse_dir_internal(input, output, &LinkDefs::default())
    }
}

fn parse_dir_internal<P: AsRef<Path>>(
    input: P,
    output: P,
    links: &LinkDefs,
) -> std::io::Result<()> {
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
            if let Ok(s) = fs::read_to_string(input.join("header.md")) {
                let mut orig = (*links).clone();
                let new: LinkDefs;
                get_links_of!(&s, new);
                orig.extend(new.clone());

                parse_dir_internal(&i, &o, &LinkDefs(orig))?;
            } else {
                parse_dir_internal(&i, &o, links)?;
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
                let output = parse(&s, &links);
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
pub fn parse_dir_watch<P: AsRef<Path>>(input: P, output: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&input.as_ref(), notify::RecursiveMode::Recursive)?;

    info!("Watching for changes in '{}'", input.as_ref().display());

    while let Ok(mut event) = rx.recv() {
        use notify::EventKind::*;

        // wait 50ms to avoid duplicate events
        std::thread::sleep(std::time::Duration::from_millis(50));

        // drain the channel
        while let Ok(e) = rx.try_recv() {
            match e {
                Ok(e) if matches!(e.kind, Create(_) | Modify(_) | Remove(_)) => event = Ok(e),
                Err(e) => error!("Error watching directory: {}", e),
                _ => (),
            }
        }

        match event {
            Ok(event) if matches!(event.kind, Create(_) | Modify(_) | Remove(_)) => {
                info!("Change detected, reparsing...");
                if let Err(e) = parse_dir(input.as_ref(), output.as_ref()) {
                    error!("Error parsing directory: '{}'", e);
                }
            }
            Err(e) => error!("Error watching directory: {}", e),
            _ => (),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        link_concat::{get_links_of, LinkDefs},
        parse::parse,
    };

    #[test]
    fn test_links() {
        let links: LinkDefs;
        get_links_of!(
            r#"  
[link1]: /link1 "link1"   
  [link2]: /link2 "link2" 
[link3]:   
       /link3       
[link4]:   
   </link 4>        
         'link4'  
        "#,
            links
        );
        let mut flat = links
            .iter()
            .map(|(k, v)| (*k, v.url, v.title))
            .collect::<Vec<_>>();
        flat.sort();

        assert_eq!(
            flat,
            [
                ("link1", "/link1", "link1"),
                ("link2", "/link2", "link2"),
                ("link3", "/link3", ""),
                ("link4", "/link 4", "link4")
            ]
        )
    }

    #[test]
    fn test_link_concat() {
        let links: LinkDefs;
        get_links_of!(
            "[search]: /search?q=\n\n\
             [wiki]: /wiki",
            links
        );
        let s = "[wiki+animation] [search.whyistheskyblue]\n\n\
                 [animation]: /animation/Animation.html";
        let s = parse(s, &links);
        assert_eq!(
            s,
            "<p><a href=\"/wiki/animation/Animation.html\">wiki+animation</a> \
             <a href=\"/search?q=whyistheskyblue\">search.whyistheskyblue</a></p>\n"
        )
    }
}

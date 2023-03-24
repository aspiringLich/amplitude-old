use std::{io, path::Path, fs};

use pulldown_cmark::{html, Options};

use crate::link_concat::{link_concat_events, parse_markdown_link_defs, LinkMap};

/// Parse the input file and write the output to the output file.
pub fn parse(links: &LinkMap, input_path: &Path, output_path: &Path) -> io::Result<()> {
    let input = fs::read_to_string(input_path)?;
    let events = link_concat_events(&input, Options::all(), links);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events);

    fs::write(output_path, html_output)?;
    Ok(())
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
/// root
/// ├── header.md
/// ├── file1.md ⟵ these files will have the root/header.md links
/// ├── file2.md ⟵
/// ├── gaming
/// │   ├── header.md
/// │   ├── file3.md ⟵ these files will have both header.md links
/// │   └── file4.md ⟵ root/gaming/header.md will take priority
/// ...
/// ```
///
/// ## Notes on Behavior
///
///  - `.md` files will be parsed and converted to `.html` files
///  - `.md` files in the output directory will be removed
///  - `header.md` does not appear in the output
/// ```
pub fn parse_dir(input: &Path, output: &Path) -> std::io::Result<()> {
    if let Ok(s) = fs::read_to_string(input.join("header.md")) {
        let links = parse_markdown_link_defs(&s);
        parse_dir_internal(input, output, links)
    } else {
        parse_dir_internal(input, output, LinkMap::new())
    }
}

fn parse_dir_internal(input: &Path, output: &Path, links: LinkMap) -> std::io::Result<()> {
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
            "md" => fs::remove_file(o)?,
            _ => {
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
            fs::create_dir(&o)?;
            if let Ok(s) = fs::read_to_string(input.join("header.md")) {
                let mut new_links = links.clone();
                new_links.extend(parse_markdown_link_defs(&s));
                parse_dir_internal(&i, &o, new_links)?;
            } else {
                parse_dir_internal(&i, &o, links.clone())?;
            }
            continue;
        }

        let ext = i.extension().unwrap_or_default();
        let name = i.file_name().unwrap_or_default();

        // if its a markdown file
        if ext == "md" {
            // ignore header.md
            if name == "header.md" {
                continue;
            }

            // otherwise, parse
            let s = fs::read_to_string(i)?;
            let events = link_concat_events(&s, Options::all(), &links);

            let mut html_out = String::new();
            html::push_html(&mut html_out, events);

            let path = o.with_extension("html");
            fs::write(path, html_out)?;
        }
        // else
        else {
            fs::copy(i, o)?;
        }
    }

    Ok(())
}

/// This function will watch the input directory and write to the output
/// directory when detecting file changes using the `notify` crate.
///
/// See [`parse_dir_recursive`] for more description on how this function
/// behaves
pub fn parse_dir_watch(_input_dir: &Path, _output_path: &Path) {}

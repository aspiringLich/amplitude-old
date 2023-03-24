/// This module contains the code for concatenating together
/// markdown links
use std::{collections::HashMap, vec::IntoIter};

use pulldown_cmark::{BrokenLink, CowStr, Event, Options};

/// Contains information representing a Link Definition
#[derive(Debug)]
pub struct LinkDef<'a> {
    pub url: &'a str,
    pub title: &'a str,
}

pub type LinkMap<'a> = HashMap<&'a str, LinkDef<'a>>;

/// Generates a list of events using the given links and link concat callback
///
/// ```
/// # use parser::link_concat::{link_concat_events, parse_markdown_link_defs};
/// # use pulldown_cmark::{html, Options};
/// let link_header = "[search]: https://google.com/search?q=";
/// let text = "[search][search+test]";
///
/// let events = link_concat_events(text, Options::empty(), parse_markdown_link_defs(link_header));
///
/// let mut html_output = String::new();
/// html::push_html(&mut html_output, events);
///
/// assert_eq!(html_output, "<p><a href=\"https://google.com/search?q=test\">search</a></p>\n");
/// ```
pub fn link_concat_events<'a>(
    text: &'a str,
    options: Options,
    links: LinkMap<'a>,
) -> IntoIter<Event<'a>> {
    let mut callback = |link: BrokenLink| {
        let (first, second) = link.reference.split_once('+')?;
        let first = links.get(first)?;
        Some((
            CowStr::Boxed((first.url.to_owned() + second).into_boxed_str()),
            CowStr::Borrowed(first.title),
        ))
    };
    // this is just for convenience when passing into `html::push_html`
    pulldown_cmark::Parser::new_with_broken_link_callback(text, options, Some(&mut callback))
        .collect::<Vec<_>>()
        .into_iter()
}

/// Following the [commonmark spec](https://spec.commonmark.org/0.18/#link-reference-definitions),
/// parse a file for its Link Reference Definitions.
///
/// Does not expect anything other than the link reference definitions, so although itll try and
/// ignore other things, it may not work exactly like the Commonmark spec
///
/// ALSO I do not account for multi-line link defs because im lazy
pub fn parse_markdown_link_defs<'a>(input: &'a str) -> LinkMap<'a> {
    let mut iter = input.split('\n').peekable();
    let mut out = HashMap::new();

    let mut prev_empty_or_link = true;
    while let Some(line) = iter.next() {
        // if the previous line is empty
        if line.trim_start().is_empty() {
            prev_empty_or_link = true;
            continue;
        }

        // bail out of the loop, as this isnt a link
        //
        // shhh its fine,,,
        macro_rules! bail {
            () => {{
                prev_empty_or_link = false;
                continue;
            }};
        }

        // if the previous line wasnt empty or a link, then exit
        if !prev_empty_or_link {
            continue;
        }

        let mut i = 1;
        let mut chars = line.chars();

        // look to make sure the line starts with a `[` indented
        // by 0-3 spaces
        while let Some(c) = chars.next() {
            match c {
                '[' => break,
                ' ' if i <= 3 => i += 1,
                _ => {
                    bail!();
                }
            }
        }

        // get the name of the link
        let line = &line[i..];
        let Some(split) = line.find(']') else { bail!() };
        let (name, mut line) = line.split_at(split);
        
        // commonmark spec says the first link definition takes priority
        if out.contains_key(name) {
            bail!();
        }

        if line.chars().nth(1) != Some(':') {
            bail!()
        }

        line = &line[2..].trim_start();
        let split = line.find(|c: char| c.is_whitespace());

        // yes title, get both
        let (url, title) = if let Some(split) = split {
            let (url, title) = line.split_at(split);
            (url, title.trim())
        }
        // no title, just get the url
        else {
            (line, "")
        };

        let title = match title.trim_start() {
            "" => "",
            e => {
                // assert that its surrounded by quotes
                match (e.bytes().next(), e.bytes().last()) {
                    (Some(b'\''), Some(b'\'')) | (Some(b'"'), Some(b'"')) => {}
                    _ => bail!(),
                }
                &e[1..e.len() - 1]
            }
        };

        // finally, add the link def to out
        out.insert(name, LinkDef { url, title });
    }
    out
}

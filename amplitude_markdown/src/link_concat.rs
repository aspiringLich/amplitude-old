/// This module contains the code for concatenating together
/// markdown links
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

use pulldown_cmark::{BrokenLink, CowStr, Event, Options, Parser, RefDefs};
use tracing::warn;

/// Contains information representing a Link Definition
#[derive(Debug, Clone)]
pub struct LinkDef<'a> {
    pub url: &'a str,
    pub title: &'a str,
}

/// Generates a list of events using the given links and link concat callback
pub(crate) fn link_concat_events<'a>(
    text: &'a str,
    options: Options,
    links: &'a LinkDefs<'a>,
    other: &'a LinkDefs<'a>,
) -> Vec<Event<'a>> {
    let mut callback = move |link: BrokenLink| -> Option<(CowStr, CowStr)> {
        // adding two links together
        if let Some((first, second)) = link.reference.split_once('+') {
            let first = links.get(first).or(other.get(first))?;
            let second = links.get(second).or(other.get(second))?;
            return Some((
                CowStr::Boxed((first.url.to_string() + second.url).into_boxed_str()),
                CowStr::Borrowed(first.title),
            ));
        }
        // adding a link and a string together
        if let Some((first, second)) = link.reference.split_once('/') {
            let first = links.get(first).or(other.get(first))?;
            return Some((
                CowStr::Boxed((first.url.to_string() + "/" + second).into_boxed_str()),
                CowStr::Borrowed(first.title),
            ));
        }
        // adding a link and a string together without the slash
        if let Some((first, second)) = link.reference.split_once('.') {
            let first = links.get(first).or(other.get(first))?;
            return Some((
                CowStr::Boxed((first.url.to_string() + second).into_boxed_str()),
                CowStr::Borrowed(first.title),
            ));
        } else {
            return None;
        }
    };
    pulldown_cmark::Parser::new_with_broken_link_callback(text, options, Some(&mut callback))
        .collect::<Vec<_>>()
}

#[derive(Default)]
pub(crate) struct LinkDefs<'a>(pub(crate) HashMap<&'a str, LinkDef<'a>>);

impl<'a> Deref for LinkDefs<'a> {
    type Target = HashMap<&'a str, LinkDef<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for LinkDefs<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<&'a RefDefs<'a>> for LinkDefs<'a> {
    fn from(refs: &'a RefDefs<'a>) -> Self {
        let mut map = HashMap::new();
        for (key, value) in refs.iter() {
            if key.contains('+') || key.contains('/') || key.contains('.') {
                warn!("Link definitions should not contain '+', '/', or '.' characters, as they are used for concatenation");
            }
            map.insert(
                key,
                LinkDef {
                    url: value.dest.as_ref(),
                    title: value.title.as_ref().map(|s| s.as_ref()).unwrap_or(""),
                },
            );
        }
        Self(map)
    }
}

/// This is normally impossible due to lifetime limits, so macro it is
pub(crate) macro get_links_of($text:expr, $links:ident) {
    let parser = Parser::new($text);
    $links = parser.reference_definitions().into();
}

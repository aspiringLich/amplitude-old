/// This module contains the code for concatenating together
/// markdown links
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use pulldown_cmark::{BrokenLink, CowStr, Parser, RefDefs};
use tracing::warn;

/// Contains information representing a Link Definition
#[derive(Debug, Clone)]
pub struct LinkDef<'a> {
    pub url: &'a str,
    pub title: &'a str,
}

/// Generates a list of events using the given links and link concat callback
pub(crate) fn link_concat_callback<'a>(
    link: BrokenLink,
    links: &'a LinkDefs<'a>,
) -> Option<(CowStr<'a>, CowStr<'a>)> {
    // adding two links together
    if let Some((first, second)) = link.reference.split_once('+') {
        let first = links.get(first)?;
        let second = links.get(second)?;
        Some((
            CowStr::Boxed((first.url.to_string() + second.url).into_boxed_str()),
            CowStr::Borrowed(first.title),
        ))
    }
    // adding a link and a string together
    else if let Some((first, second)) = link.reference.split_once('/') {
        let first = links.get(first)?;
        Some((
            CowStr::Boxed((first.url.to_string() + "/" + second).into_boxed_str()),
            CowStr::Borrowed(first.title),
        ))
    }
    // adding a link and a string together without the slash
    else if let Some((first, second)) = link.reference.split_once('.') {
        let first = links.get(first)?;
        Some((
            CowStr::Boxed((first.url.to_string() + second).into_boxed_str()),
            CowStr::Borrowed(first.title),
        ))
    } else {
        None
    }
}

#[derive(Default, Clone)]
pub(crate) struct LinkDefs<'a>(pub(crate) HashMap<&'a str, LinkDef<'a>>);

impl<'a> LinkDefs<'a> {
    pub fn extend(&mut self, parser: &'a Parser<'a, 'a>) {
        for (k, v) in parser.reference_definitions().iter() {
            self.insert(
                k,
                LinkDef {
                    url: &v.dest,
                    title: v.title.as_ref().unwrap_or(&CowStr::Borrowed("")),
                },
            );
        }
    }
}

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

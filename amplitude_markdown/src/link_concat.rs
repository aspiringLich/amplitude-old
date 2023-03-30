/// This module contains the code for concatenating together
/// markdown links
use comrak::RefMap;

/// A link concatenation callback
pub(crate) fn link_concat_callback(link: &str, links: &RefMap) -> Option<(String, String)> {
    // Add a to b, parsing them as links if they are links
    if let Some((left, right)) = link.split_once(|c| matches!(c, '+')) {
        let title = links
            .map
            .get(left)
            .map(|l| l.title.clone())
            .unwrap_or_default();
        let l = links.map.get(left).map(|l| l.url.as_str()).unwrap_or(left);
        let r = links
            .map
            .get(right)
            .map(|l| l.url.as_str())
            .unwrap_or(right);
        Some((l.to_string() + r, title))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use comrak::Arena;

    #[test]
    fn test_link_concat() {
        use crate::link_concat::link_concat_callback;

        let links = "
[a]: https://a.com
[b]: https://b.com
[c]: https://c.com
";
        let mut refs = comrak::parse_document_refs(&Arena::new(), links);
        let flatten = refs.map.iter().map(|(l, r)| (l.as_str(), r.url.as_str()));
        let mut list = flatten.collect::<Vec<_>>();
        list.sort();
        assert_eq!(
            list,
            [
                ("a", "https://a.com"),
                ("b", "https://b.com"),
                ("c", "https://c.com")
            ]
        );

        let arena = Arena::new();
        let out = comrak::parse_document_with_broken_link_callback(
            &arena,
            "[a+/search?q=b]",
            &comrak::ComrakOptions::default(),
            Some(&mut |link| link_concat_callback(link, &mut refs)),
        );
        let mut cm = vec![];
        comrak::format_commonmark(out, &comrak::ComrakOptions::default(), &mut cm).unwrap();

        assert_eq!(
            String::from_utf8(cm).unwrap(),
            "[a+/search?q=b](https://a.com/search?q=b)\n"
        );
    }
}

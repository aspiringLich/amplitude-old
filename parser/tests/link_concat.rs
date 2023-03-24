use parser::link_concat::{link_concat_events, parse_markdown_link_defs};
use pulldown_cmark::{html, Options};

#[test]
fn test_parse_markdown_link_defs() {
    let markdown_input = "
 [test]: test  'test'
[test2]: test2
  [test3]:   test3  'ee'
[fail]: fail 'fail
[fail2]: fail2 dasdk
";
    let map = parse_markdown_link_defs(markdown_input);
    dbg!(&map);

    // flatten the map into something easier to compare
    let mut flat = map
        .iter()
        .map(|(k, v)| (*k, v.url, v.title))
        .collect::<Vec<_>>();
    flat.sort();

    assert_eq!(
        flat,
        [
            ("test", "test", "test"),
            ("test2", "test2", ""),
            ("test3", "test3", "ee"),
        ]
    )
}

#[test]
fn test_link_concat() {
    let text = "
[test]: test/

[gaming][test+subtest]
";
    let events = link_concat_events(text, Options::empty(), text);

    let mut html_output = String::new();
    html::push_html(&mut html_output, events);

    assert_eq!(html_output, "<p><a href=\"test/subtest\">gaming</a></p>\n");
}

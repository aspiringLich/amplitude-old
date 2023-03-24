use parser::link_concat::{broken_link_concat_callback, parse_markdown_link_defs};
use pulldown_cmark::{Options, Parser, html};

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
    let markdown_input = "
[test]: test/

[gaming][test+subtest]
";
    
    let map = parse_markdown_link_defs(markdown_input);
    let mut callback = |link| broken_link_concat_callback(&map, link);
    let parser = Parser::new_with_broken_link_callback(
        markdown_input,
        Options::all(),
        Some(&mut callback),
    );
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    assert_eq!(html_output, "<p><a href=\"test/subtest\">gaming</a></p>\n");
}

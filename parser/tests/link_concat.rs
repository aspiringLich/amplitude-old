use parser::link_concat::parse_markdown_link_defs;

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
        .map(|(k, v)| (*k, v.url, v.title.unwrap_or("")))
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

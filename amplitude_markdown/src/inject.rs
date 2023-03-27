use pulldown_cmark::Parser;

#[test]
fn test_inject() {
    let parser = Parser::new(
        "
@quiz
```
# This is a code block
```
",
    );
    panic!("{:#?}", parser.into_iter().collect::<Vec<_>>())
}

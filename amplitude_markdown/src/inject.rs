mod quiz;

use std::collections::HashMap;

use pulldown_cmark::Event;
use serde::Deserialize;

type Callback = fn(&[&str], &mut Vec<Event>) -> anyhow::Result<()>;

/// A list of tags that are expected to be found in the markdown to call the
/// callback
enum ExpectedTag {
    CodeBlock(Option<&'static str>),
    Quote,
}

#[ctor::ctor]
static INJECTION_TAGS: HashMap<String, (&'static [ExpectedTag], Callback)> = {
    let mut m = HashMap::new();
    let mut insert = |tag: &str, tags: &'static [ExpectedTag], callback: Callback| {
        m.insert(tag.to_string(), (tags, callback));
    };
    use ExpectedTag::*;
    insert("quiz", &[CodeBlock(Some("toml"))], quiz::inject_quiz);
    m
};

#[cfg(test)]
mod tests {
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
}

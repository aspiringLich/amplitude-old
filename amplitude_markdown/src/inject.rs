// mod info;
// mod quiz;

use anyhow::Context;
use comrak::nodes::{Ast, AstNode, NodeValue};
use comrak::RefMap;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;

type Callback = fn(&str, &AstNode, &mut ParseState) -> anyhow::Result<()>;

pub(crate) struct ParseState<'a> {
    pub refs: &'a RefMap,
}

/// A list of tags that are expected to be found in the markdown to call the
/// callback
#[derive(Debug)]
enum ExpectedTag {
    CodeBlock(Option<&'static str>),
    Quote,
}

impl ExpectedTag {
    fn matches(&self, node: &AstNode) -> bool {
        use ExpectedTag::*;
        let val = &node.data.borrow().value;
        match self {
            CodeBlock(Some(lang)) => match val {
                NodeValue::CodeBlock(node) => {
                    &node
                        .info
                        .split_once(|c: char| c.is_whitespace())
                        .map(|t| t.0)
                        .unwrap_or(node.info.as_str())
                        == lang
                }
                _ => false,
            },
            CodeBlock(None) => matches!(val, NodeValue::CodeBlock(_)),
            Quote => matches!(val, NodeValue::BlockQuote),
        }
    }
}

#[ctor::ctor]
static INJECTION_TAGS: HashMap<String, (ExpectedTag, Callback)> = {
    let mut m = HashMap::new();
    let mut insert = |tag: &str, expected: ExpectedTag, callback: Callback| {
        m.insert(tag.to_string(), (expected, callback));
    };
    use ExpectedTag::*;
    // insert("quiz", CodeBlock(Some("toml")), quiz::inject_quiz);
    // macro admonition($s:literal) {
    //     insert($s, Quote, |input, id, events, state| {
    //         info::inject_badge(input, id, events, state, $s)
    //     });
    // }
    // admonition!("note");
    // admonition!("info");
    // admonition!("warn");
    // admonition!("error");
    // admonition!("correct");
    // admonition!("incorrect");
    m
};

pub(crate) fn inject<'a>(node: &'a AstNode<'a>, refs: &RefMap) -> anyhow::Result<()> {
    let mut state = ParseState { refs };

    for node in node.descendants().skip(1) {
        let v = &node.data.borrow().value;
        match v {
            NodeValue::Paragraph => {
                let children: Vec<_> = node.children().collect();
                if children.len() != 1 {
                    continue;
                }
                if let NodeValue::Text(text) = &children[0].data.borrow().value {
                    if !text.starts_with('@') {
                        continue;
                    }
                    let (text, info) = text.split_once(';').unwrap_or((text, ""));
                    if let Some((expected, callback)) = INJECTION_TAGS.get(text) {
                        if expected.matches(node) {
                            callback(info, node, &mut state)
                                .context(format!("While parsing tag {text}"))?;
                        } else {
                            anyhow::bail!("Expected tag {text} to be {expected:?}");
                        }
                    } else {
                        anyhow::bail!("Unknown tag {text}");
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use std::default::default;

    // use pulldown_cmark::Parser;

    // use crate::parse::{parse_into_events, parse};

    //     #[test]
    //     fn test_inject() {
    //         let input =
    //             r#"
    // @quiz
    // ```toml
    // question = """
    // But like *are* you gay?
    // """

    // [[answers]]
    // text = "answer"
    // response = "woo"
    // ```"#;
    //         panic!("{:#?}", parse(input, &default()))
    //         // panic!("{:#?}", parse_into_events(parser, &default()).unwrap().into_iter().collect::<Vec<_>>())
    //     }
}

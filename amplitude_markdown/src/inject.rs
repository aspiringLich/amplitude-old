// mod info;
pub mod quiz;

use anyhow::Context;
use comrak::nodes::{AstNode, NodeValue};
use comrak::RefMap;
use std::collections::HashMap;

use crate::parse::ParseState;

type Callback = fn(ArticleRef, &str, &AstNode, &mut ParseState, &RefMap) -> anyhow::Result<()>;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct ArticleRef<'a> {
    pub article: &'a str,
    pub course: &'a str,
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

fn display_node(node: &AstNode) -> String {
    let val = &node.data.borrow().value;
    use NodeValue::*;
    match val {
        CodeBlock(_) => "CodeBlock(_)",
        BlockQuote => "BlockQuote",
        Document => "Document",
        FrontMatter(_) => "FrontMatter(_)",
        List(_) => "List(_)",
        Item(_) => "Item(_)",
        DescriptionList => "DescriptionList",
        DescriptionItem(_) => "DescriptionItem(_)",
        DescriptionTerm => "DescriptionTerm",
        DescriptionDetails => "DescriptionDetails",
        HtmlBlock(_) => "HtmlBlock(_)",
        Paragraph => "Paragraph",
        Heading(_) => "Heading(_)",
        ThematicBreak => "ThematicBreak",
        FootnoteDefinition(_) => "FootnoteDefinition(_)",
        Table(_) => "Table(_)",
        TableRow(_) => "TableRow(_)",
        TableCell => "TableCell ",
        Text(_) => "Text(_)",
        TaskItem { .. } => "TaskItem { .. }",
        SoftBreak => "SoftBreak",
        LineBreak => "LineBreak",
        Code(_) => "Code(_)",
        HtmlInline(_) => "HtmlInline(_)",
        Emph => "Emph ",
        Strong => "Strong ",
        Strikethrough => "Strikethrough",
        Superscript => todo!(),
        Link(_) => "Link(_)",
        Image(_) => "Image(_)",
        FootnoteReference(_) => "FootnoteReference(_)",
    }
    .to_string()
}

#[ctor::ctor]
static INJECTION_TAGS: HashMap<String, (ExpectedTag, Callback)> = {
    let mut m = HashMap::new();
    let mut insert = |tag: &str, expected: ExpectedTag, callback: Callback| {
        m.insert(tag.to_string(), (expected, callback));
    };
    use ExpectedTag::*;
    insert("quiz", CodeBlock(Some("toml")), quiz::inject_quiz);
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

pub(crate) fn inject<'a>(
    article: ArticleRef,
    node: &'a AstNode<'a>,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<()> {
    for node in node.descendants().skip(1) {
        let v = &node.data.borrow().value;
        match v {
            NodeValue::Paragraph => {
                let children: Vec<_> = node.children().collect();
                if children.len() != 1 {
                    continue;
                }
                let val = &children[0].data.borrow().value;
                if let NodeValue::Text(text) = &val {
                    if !text.starts_with('@') {
                        continue;
                    }
                    let (text, info) = text.split_once(';').unwrap_or((text, ""));
                    if let Some((expected, callback)) = INJECTION_TAGS.get(&text[1..]) {
                        let n = node
                            .next_sibling()
                            .context(format!("Unexpected end of AST after tag {text}"))?;
                        node.detach();
                        if expected.matches(n) {
                            callback(article.clone(), info, n, state, refs)
                                .context(format!("While parsing tag {text}"))?;
                        } else {
                            anyhow::bail!(
                                "Expected tag {text} to come before {expected:?}, found {}",
                                display_node(&n)
                            );
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

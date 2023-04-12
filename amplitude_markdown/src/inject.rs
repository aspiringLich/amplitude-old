mod admonition;
mod code;
mod quiz;

use anyhow::Context;

use comrak::nodes::{AstNode, NodeValue};
use comrak::RefMap;

use std::collections::HashMap;
use std::path::Path;

use comrak::html;

use crate::state::ParseState;

type Callback = for<'a> fn(
    &Path,
    &HashMap<String, String>,
    &'a AstNode<'a>,
    &mut ParseState,
    &RefMap,
) -> anyhow::Result<Vec<&'a AstNode<'a>>>;

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

struct CallbackInfo {
    expected: ExpectedTag,
    callback: Callback,
    /// The keys that are expected to be present in the tag
    /// bool for if the key is mandatory
    keys: HashMap<&'static str, bool>,
}

impl CallbackInfo {
    fn new(
        expected: ExpectedTag,
        callback: Callback,
        expected_keys: &[&'static str],
        mandatory_keys: &[&'static str],
    ) -> Self {
        Self {
            expected,
            callback,
            keys: expected_keys
                .iter()
                .map(|s| (*s, false))
                .chain(mandatory_keys.iter().map(|s| (*s, true)))
                .collect(),
        }
    }
}

#[ctor::ctor]
static INJECTION_TAGS: HashMap<String, CallbackInfo> = {
    let mut m = HashMap::new();
    let mut insert = |tag: &str, info: CallbackInfo| {
        m.insert(tag.to_string(), info);
    };
    use ExpectedTag::*;
    insert(
        "@quiz",
        CallbackInfo::new(CodeBlock(Some("toml")), quiz::inject_quiz, &[], &["id"]),
    );
    insert(
        "@!",
        CallbackInfo::new(
            Quote,
            admonition::inject_admonition,
            &["note", "info", "warning", "success", "failiure"],
            &[],
        ),
    );
    insert(
        "@code",
        CallbackInfo::new(CodeBlock(None), code::inject_code, &[], &[]),
    );
    m
};

fn parse_args(input: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for word in input.split(|c: char| c.is_whitespace()) {
        let (key, value) = word.split_once('=').unwrap_or((word, ""));
        out.insert(key.to_string(), value.to_string());
    }
    out
}

pub(crate) fn inject<'a>(
    article: &'a Path,
    node: &'a AstNode<'a>,
    refs: &RefMap,
    state: &'a mut ParseState,
) -> anyhow::Result<()> {
    // variables were going to detach
    let mut to_detach = vec![];
    // dbg!(node);
    for node in node.descendants() {
        let v = &node.data.borrow().value;
        if !matches!(v, NodeValue::Paragraph) {
            continue;
        }

        let children: Vec<_> = node.children().collect();
        if children.len() != 1 {
            continue;
        }
        let val = &children[0].data.borrow().value;
        if let NodeValue::Text(text) = &val {
            if !text.starts_with('@') {
                continue;
            }
            let (text, post) = text
                .split_once(|c: char| c.is_whitespace())
                .unwrap_or((text, ""));

            if let Some(info) = INJECTION_TAGS.get(text) {
                let args = parse_args(post);
                for (key, mandatory) in &info.keys {
                    if !args.contains_key(*key) && *mandatory {
                        anyhow::bail!(
                            "Missing mandatory key `{key}` in tag `{text}`",
                            key = key,
                            text = text
                        );
                    }
                }
                for arg in &args {
                    if !arg.1.is_empty() && !info.keys.contains_key(arg.0.as_str()) {
                        anyhow::bail!(
                            "Unexpected key `{key}` in tag `{text}`",
                            key = arg.0,
                            text = text
                        );
                    }
                }

                let n = node
                    .next_sibling()
                    .context(format!("Unexpected end of AST after tag `{text}`"))?;
                to_detach.push(node);
                let expected = &info.expected;
                if expected.matches(n) {
                    let mut ret = (info.callback)(article, &args, n, state, refs)
                        .context(format!("while calling callback for tag `{text}`"))?;
                    to_detach.append(&mut ret);
                } else {
                    anyhow::bail!(
                        "Expected tag `{text}` to come before {expected:?}, found {}",
                        display_node(n)
                    );
                }
            } else {
                anyhow::bail!("Unknown tag `{text}`");
            }
        }
    }

    for node in to_detach {
        node.detach();
    }

    Ok(())
}

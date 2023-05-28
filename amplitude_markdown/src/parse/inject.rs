use std::collections::HashMap;

use super::context::ItemContext;

use anyhow::Context;
use comrak::{
    html,
    nodes::{AstNode, NodeValue},
};

mod admonition;
mod code;
mod quiz;
mod utils;

type CallbackArgs = HashMap<String, String>;
type CallbackRet<'a> = anyhow::Result<Vec<&'a AstNode<'a>>>;

trait DynCallback: Send + Sync + 'static {
    fn run_callback<'a>(
        &self,
        args: CallbackArgs,
        node: &'a AstNode<'a>,
        ctx: &mut ItemContext,
    ) -> CallbackRet<'a>;

    fn marker(&self) -> &'static str;
    fn expected_tag(&self) -> ExpectedTag;
    fn mandatory_keys(&self) -> &'static [&'static str];
    fn optional_keys(&self) -> &'static [&'static str];
}

pub trait Callback: Send + Sync + 'static {
    fn run_callback<'a>(
        &self,
        args: CallbackArgs,
        node: &'a AstNode<'a>,
        ctx: &mut ItemContext,
    ) -> CallbackRet<'a>;

    const MARKER: &'static str;
    const EXPECTED_TAG: ExpectedTag;
    const MANDATORY_KEYS: &'static [&'static str] = &[];
    const OPTIONAL_KEYS: &'static [&'static str] = &[];
}

impl<T: Callback> DynCallback for T {
    fn run_callback<'a>(
        &self,
        args: CallbackArgs,
        node: &'a AstNode<'a>,
        ctx: &mut ItemContext,
    ) -> CallbackRet<'a> {
        self.run_callback(args, node, ctx)
    }

    fn marker(&self) -> &'static str {
        Self::MARKER
    }

    fn expected_tag(&self) -> ExpectedTag {
        Self::EXPECTED_TAG
    }

    fn mandatory_keys(&self) -> &'static [&'static str] {
        Self::MANDATORY_KEYS
    }

    fn optional_keys(&self) -> &'static [&'static str] {
        Self::OPTIONAL_KEYS
    }
}

const CALLBACKS: &[&'static dyn DynCallback] = &[&admonition::Admonition];
#[ctor::ctor]
static MARKERS: HashMap<&'static str, &'static dyn DynCallback> = {
    let mut tags = HashMap::new();
    for callback in CALLBACKS {
        tags.insert(callback.marker(), *callback);
    }
    tags
};

/// A list of tags that are expected to be found in the markdown to call the
/// callback
#[derive(Debug)]
pub enum ExpectedTag {
    CodeBlock(Option<&'static str>),
    BlockQuote,
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
            BlockQuote => matches!(val, NodeValue::BlockQuote),
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
        Superscript => "Superscript",
        Link(_) => "Link(_)",
        Image(_) => "Image(_)",
        FootnoteReference(_) => "FootnoteReference(_)",
    }
    .to_string()
}

fn parse_args(input: &str) -> anyhow::Result<HashMap<String, String>> {
    let mut out = HashMap::new();

    let words = input.split_whitespace().collect::<Vec<_>>();
    let mut i = 0;

    while i < words.len() {
        if words[i] == "=" {
            out.insert(
                words
                    .get(i - 1)
                    .context("`=` at beginning is not allowed")?
                    .to_string(),
                words
                    .get(i + 1)
                    .context("`=` at end is not allowed")?
                    .to_string(),
            );
            i += 2;
        } else {
            let (l, r) = words[i].split_once('=').unwrap_or((words[i], ""));
            out.insert(l.to_string(), r.to_string());
            i += 1;
        }
    }
    Ok(out)
}

pub(crate) fn inject<'a>(node: &'a AstNode<'a>, ctx: &mut ItemContext) -> anyhow::Result<()> {
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

            if let Some(info) = MARKERS.get(text) {
                let args = parse_args(post).context("While parsing arguments")?;
                for key in info.mandatory_keys() {
                    if !args.contains_key(*key) {
                        anyhow::bail!(
                            "Missing mandatory key `{key}` in tag `{text}`",
                            key = key,
                            text = text
                        );
                    }
                }
                for arg in &args {
                    if !info.mandatory_keys().contains(&arg.0.as_str())
                        && !info.optional_keys().contains(&arg.0.as_str())
                    {
                        anyhow::bail!(
                            "Unknown key `{key}` in tag `{text}`",
                            key = arg.0,
                            text = text
                        );
                    }
                }

                let n = node
                    .next_sibling()
                    .with_context(|| format!("Unexpected end of AST after tag `{text}`"))?;

                to_detach.push(node);
                let expected = &info.expected_tag();
                if expected.matches(n) {
                    let mut ret = info
                        .run_callback(args, n, ctx)
                        .with_context(|| format!("while calling callback for tag `{text}`"))?;
                    to_detach.append(&mut ret);
                } else {
                    anyhow::bail!(
                        "Expected tag `{text}` to come before {expected:?}, found {}",
                        display_node(n)
                    );
                }
            }
            // else {
            //     anyhow::bail!("Unknown tag `{text}`");
            // }
        }
    }

    for node in to_detach {
        node.detach();
    }

    Ok(())
}

#[test]
pub fn test_parse_args() {
    let test = |s, test: &[(&str, &str)]| {
        let args = parse_args(s).unwrap();
        let mut list = args
            .iter()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect::<Vec<_>>();
        list.sort();
        assert_eq!(list, test)
    };
    test("a b c", &[("a", ""), ("b", ""), ("c", "")]);
    test("a b=c", &[("a", ""), ("b", "c")]);

    test(
        "a b c d=e f = g h",
        &[
            ("a", ""),
            ("b", ""),
            ("c", ""),
            ("d", "e"),
            ("f", "g"),
            ("h", ""),
        ],
    );
}

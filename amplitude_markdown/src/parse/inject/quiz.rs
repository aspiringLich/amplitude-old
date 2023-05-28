use crate::items::{self, ItemType};

use super::*;

pub struct Quiz;

impl Callback for Quiz {
    fn run_callback<'a>(
        &self,
        args: CallbackArgs,
        node: &'a AstNode<'a>,
        ctx: &mut DataContext,
    ) -> CallbackRet<'a> {
        let id = &args["id"];
        // dbg!(node);

        let mut ast = node.data.borrow_mut();
        let NodeValue::CodeBlock(ref code) = ast.value else { anyhow::bail!("Expected Code block") };

        let quiz = items::quiz::Quiz::from_str(&code.literal, id.clone(), ctx)?;
        ctx.add_item(ItemType::Quiz(quiz), "")?;

        ast.value = NodeValue::HtmlInline(format!("<Quiz id=\"{id}\"></Quiz>\n"));

        Ok(node.children().collect())
    }

    const MARKER: &'static str = "@quiz";
    const EXPECTED_TAG: ExpectedTag = ExpectedTag::CodeBlock(Some("toml"));
    const MANDATORY_KEYS: &'static [&'static str] = &["id"];
}

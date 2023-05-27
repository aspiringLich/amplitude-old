use super::*;

pub struct Admonition;

impl Callback for Admonition {
    fn run_callback<'a>(
        &mut self,
        args: CallbackArgs,
        node: &'a AstNode<'a>,
        ctx: &mut ItemContext,
    ) -> CallbackRet<'a> {
        let mut out = vec![];
        html::format_document(node, ctx.markdown_options(), &mut out)
            .context("failed to parse admonition")?;

        anyhow::ensure!(args.len() == 1, "admonition must be provided with a type");
        let tag = args.keys().next().unwrap();

        let s = String::from_utf8(out)
            .context("failed to parse admonition output into valid string")?;
        let html = s
            .strip_prefix("<blockquote>")
            .and_then(|s| s.strip_suffix("</blockquote>\n"))
            .context("expected blockquote tags in html")?;
        let mut data = node.data.borrow_mut();
        data.value =
            NodeValue::HtmlInline(format!("<Admonition type=\"{tag}\">{html}</Admonition>\n",));

        Ok(node.children().collect())
    }

    const MARKER: &'static str = "@!";
    const EXPECTED_TAG: ExpectedTag = ExpectedTag::BlockQuote;
    const OPTIONAL_KEYS: &'static [&'static str] =
        &["note", "info", "warning", "success", "failiure"];
}

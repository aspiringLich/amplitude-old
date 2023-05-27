use super::*;

pub fn inject_admonition<'a>(
    _: &ArticleConfig,
    args: &HashMap<String, String>,
    node: &'a AstNode<'a>,
    ctx: &mut ItemContext,
    _: &RefMap,
) -> anyhow::Result<Vec<&'a AstNode<'a>>> {
    let mut out = vec![];
    html::format_document(node, &ctx.options(), &mut out).context("failed to parse admonition")?;

    anyhow::ensure!(args.len() == 1, "admonition must be provided with a type");
    let tag = args.keys().next().unwrap();

    let s =
        String::from_utf8(out).context("failed to parse admonition output into valid string")?;
    let html = s
        .strip_prefix("<blockquote>")
        .and_then(|s| s.strip_suffix("</blockquote>\n"))
        .context("expected blockquote tags in html")?;
    let mut data = node.data.borrow_mut();
    data.value =
        NodeValue::HtmlInline(format!("<Admonition type=\"{tag}\">{html}</Admonition>\n",));

    Ok(node.children().collect())
}

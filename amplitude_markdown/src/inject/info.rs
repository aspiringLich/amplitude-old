use super::*;

pub(super) fn inject_badge(
    input: Vec<Event>,
    _: &str,
    events: &mut Vec<Event>,
    _: &mut ParseState,
    str: &'static str,
) -> anyhow::Result<()> {
    let mut html = String::new();
    html::push_html(
        &mut html,
        input[1..input.len() - 1].into_iter().map(|x| x.clone()),
    );

    let c = str.chars().next().context("Empty str!")?;
    let c = c.to_uppercase().next().unwrap_or(c);
    let title = c.to_string() + &str[1..];
    events.push(Event::Html(CowStr::Boxed(
        format!("<blockquote id=\"{str}\">\n{title}<br>\n{html}</blockquote>").into_boxed_str(),
    )));

    Ok(())
}

use crate::parse::context::MarkdownContext;

pub struct MarkdownDeserializer<'a> {
    pub deserializer: toml::Deserializer<'a>,
    pub context: &'a MarkdownContext<'a>,
}

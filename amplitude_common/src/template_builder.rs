use anyhow::Context;
use tracing::warn;

/// A struct to facilitate turning a HTML template file
/// into the renderred HTML file
///
/// ```
/// # use amplitude_common::template_builder::TemplateBuilder;
/// let template = "<p>{{content}}</p>";
///
/// let output = TemplateBuilder::new(template)
///     .unwrap()
///     .replace("content", "Hello, world!")
///     .build();
///
/// assert_eq!(output, "<p>Hello, world!</p>");
/// ```
#[derive(Debug, Clone, Default)]
pub struct TemplateBuilder<'a> {
    sections: Vec<&'a str>,
    snippets: Vec<&'a str>,
    changed: Vec<Option<String>>,
}

impl<'a> TemplateBuilder<'a> {
    /// Creates a new TemplateBuilder from a template string
    pub fn new(template: &'a str) -> anyhow::Result<Self> {
        let mut out = Self::default();

        let mut split = template.split("{{");
        out.sections
            .push(split.next().context("Expected file to not be empty")?);
        for section in split {
            let mut index = None;
            let mut iter = section.chars().enumerate();
            loop {
                let Some((i, c)) = iter.next() else {
                    break;
                };

                if c.is_whitespace() {
                    break;
                }
                if c == '}' && let Some((_, '}')) = iter.next() {
                    index = Some(i);
                    break;
                }
            }

            if let Some(i) = index {
                let snippet = section.get(0..i);
                let section = section.get(i + 2..);

                out.sections
                    .push(section.context("error when getting section str")?);
                out.snippets
                    .push(snippet.context("error when getting snippet str")?);
            }
        }

        out.changed = vec![None; out.snippets.len()];

        Ok(out)
    }

    /// Replaces all occurences of `{{from}}` with `to`
    pub fn replace<T: ToString>(mut self, from: &str, to: T) -> Self {
        let with_string = to.to_string();

        let mut any = false;

        for (i, s) in self.snippets.iter().enumerate() {
            if *s == from {
                any = true;
                self.changed[i] = Some(with_string.clone());
            }
        }

        if !any {
            warn!("Did not find {{{{{}}}}} in the specified template!", from);
        }

        self
    }

    /// Builds the template into a string
    pub fn build(self) -> String {
        let mut out = String::new();
        out.push_str(self.sections.first().unwrap());

        for i in 0..self.snippets.len() {
            if let Some(snippet) = &self.changed[i] {
                out += snippet;
            } else {
                warn!("Unreplaced snippet: {{{{{}}}}}", self.snippets[i]);
                out += &format!("{{{{{}}}}}", self.snippets[i])
            }
            out.push_str(self.sections[i + 1]);
        }

        out
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_template_builder() {
        let template = r#"
            <html>
                <head>
                    <title>{{title}}</title>
                </head>
                <body>
                    <h1>{{title}}</h1>
                    <p>{{content}}</p>
                </body>
            </html>
        "#;

        let out = super::TemplateBuilder::new(template)
            .unwrap()
            .replace("title", "My Title")
            .replace("content", "My Content")
            .build();

        assert_eq!(
            out,
            r#"
            <html>
                <head>
                    <title>My Title</title>
                </head>
                <body>
                    <h1>My Title</h1>
                    <p>My Content</p>
                </body>
            </html>
        "#
        );
    }
}

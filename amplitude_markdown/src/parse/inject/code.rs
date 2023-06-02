use super::*;

pub struct Code;

impl Callback for Code {
    fn run_callback<'a>(
        &self,
        args: CallbackArgs,
        node: &AstNode<'a>,
        _: &mut DataContext,
    ) -> anyhow::Result<Vec<&'a AstNode<'a>>> {
        let val = &mut node.data.borrow_mut().value;
        match val {
            NodeValue::CodeBlock(ref mut code) => {
                code.fenced = true;
                code.fence_char = b'`';
                code.fence_length = 3;
                code.info = args
                    .keys()
                    .next()
                    .context("`@code` must be given a language")?
                    .to_string();
            }
            _ => unreachable!(),
        }

        Ok(vec![])
    }

    const MARKER: &'static str = "@code";
    const EXPECTED_TAG: ExpectedTag = ExpectedTag::CodeBlock(None);
    const OPTIONAL_KEYS: &'static [&'static str] = &["*"];
}

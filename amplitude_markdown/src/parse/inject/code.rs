use super::*;

pub fn inject_code<'a>(
    args: CallbackArgs,
    node: &AstNode<'a>,
    
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

use super::*;

#[derive(Debug, Serialize)]
pub struct Project;

fn parse_md(contents: &DirContents, _dir: &Path) -> anyhow::Result<()> {
    for file in contents.query_type(FileType::Markdown) {
        let num = &file.name[0..2];
        let _name = &file.name[3..];

        let _n = usize::from_str_radix(num, 10)
            .context("While trying to parse first 2 characters of filename as number")?;
    }

    Ok(())
}

impl Item for Project {
    fn parse_from_dir(
        dir: &Path,
        contents: DirContents,
        _context: &mut DataContext,
    ) -> anyhow::Result<ItemType> {
        ensure!(
            contents.query("start", FileType::Code).next().is_some(),
            "start.<code>",
            "Starting code"
        );
        ensure!(
            contents.query_type(FileType::Markdown).next().is_some(),
            "*.md"
        );
        
        parse_md(&contents, dir).context("While parsing md")?;

        let project = Project;
        Ok(ItemType::Project(project))
    }
}

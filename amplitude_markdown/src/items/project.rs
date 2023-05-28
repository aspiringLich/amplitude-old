use super::*;

#[derive(Debug)]
pub struct Project;

impl Item for Project {
    fn parse_from_dir(
        _dir: &Path,
        contents: DirContents,
        _context: &mut ItemContext,
    ) -> anyhow::Result<ItemType> {
        ensure!(
            contents.query("start", FileType::Code).next().is_some(),
            "Starting code"
        );
        ensure!(
            contents.query_type(FileType::Markdown).next().is_some(),
            "Required file(s): `*.md` not found"
        );

        let project = Project;
        Ok(ItemType::Project(project))
    }
}

use super::*;

#[derive(Debug, Serialize)]
pub struct Project;

impl Item for Project {
    fn parse_from_dir(
        _dir: &Path,
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

        let project = Project;
        Ok(ItemType::Project(project))
    }
}

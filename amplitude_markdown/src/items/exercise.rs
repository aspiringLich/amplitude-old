
use super::*;

#[derive(Debug, Serialize)]
pub struct Exercise;

impl Item for Exercise {
    fn parse_from_dir(
        _dir: &Path,
        contents: DirContents,
        _context: &mut DataContext,
    ) -> anyhow::Result<ItemType> {
        ensure!(
            contents.query("gen", FileType::Code).next().is_some(),
            "gen.<code>",
            "Test case generator"
        );
        ensure!(
            contents.query("start", FileType::Code).next().is_some(),
            "start.<code>",
            "Starting code"
        );

        let exercise = Exercise;
        Ok(ItemType::Exercise(exercise))
    }
}

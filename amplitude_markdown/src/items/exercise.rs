use std::collections::HashMap;

use super::*;

pub struct FunctionConfig {
    
}

pub struct ExcerciseConfig {
    functions: HashMap<String, FunctionConfig>,
}

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
        ensure!(contents.contains("instructions.md"), "instructions.md");

        let exercise = Exercise;
        Ok(ItemType::Exercise(exercise))
    }
}

mod tests {
    
}
use crate::PuzzleConfig;

#[derive(Debug, Clone)]
pub struct PuzzleConfigCollection {
    name: String,
    description: Option<String>,
    author: String,
    puzzles: Vec<PuzzleConfig>,
}

impl PuzzleConfigCollection {
    pub fn new(
        name: String,
        description: Option<String>,
        author: String,
        puzzles: Vec<PuzzleConfig>,
    ) -> PuzzleConfigCollection {
        PuzzleConfigCollection {
            name,
            description,
            author,
            puzzles,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn puzzles(&self) -> &Vec<PuzzleConfig> {
        &self.puzzles
    }
}

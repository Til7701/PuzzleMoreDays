use crate::puzzle::get_default_config;
use crate::puzzle::tile::Tile;
use crate::state::TargetSelection;
use ndarray::Array2;
use std::fmt::Debug;

pub trait TargetName: Debug + Send + Sync {
    fn create_target_name(&self, selection: &TargetSelection) -> Result<String, String>;
    fn box_clone(&self) -> Box<dyn TargetName>;
}

impl Clone for Box<dyn TargetName> {
    fn clone(&self) -> Box<dyn TargetName> {
        self.box_clone()
    }
}

#[derive(Debug, Clone)]
pub struct Meaning {
    pub index: i32,
    pub name: String,
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Clone)]
pub struct SolutionStatistics {
    pub min_per_meaning: i32,
    pub max_per_meaning: i32,
    pub average_per_meaning: f64,
    pub mean_per_meaning: i32,
    pub total_solutions: i32,
}

#[derive(Debug, Clone)]
pub struct PuzzleConfig {
    pub name: String,
    pub board_layout: Array2<bool>,
    pub meaning_areas: Array2<i32>,
    pub meaning_values: Array2<i32>,
    pub display_values: Array2<String>,
    pub tiles: Vec<Tile>,
    pub solution_statistics: Option<SolutionStatistics>,
    pub meaning_options: Vec<Meaning>,
    pub target_name: Box<dyn TargetName>,
}

impl PuzzleConfig {
    pub fn new(
        name: String,
        board_layout: Array2<bool>,
        meaning_areas: Array2<i32>,
        meaning_values: Array2<i32>,
        display_values: Array2<String>,
        tiles: Vec<Tile>,
        solution_statistics: Option<SolutionStatistics>,
        meaning_options: Vec<Meaning>,
        target_name: Box<dyn TargetName>,
    ) -> PuzzleConfig {
        PuzzleConfig {
            name,
            board_layout,
            meaning_areas,
            meaning_values,
            display_values,
            tiles,
            solution_statistics,
            meaning_options,
            target_name,
        }
    }

    pub fn max_meaning_value(&self) -> i32 {
        *self.meaning_values.iter().max().unwrap_or(&0)
    }
}

impl Default for PuzzleConfig {
    fn default() -> Self {
        get_default_config()
    }
}

mod bitmask;
mod config;
pub mod tile;
mod util;

pub(crate) use crate::puzzle::config::PuzzleConfig;
use crate::puzzle::config::{Meaning, SolutionStatistics, TargetName};
use crate::puzzle::util::transform;
use crate::state::{MeaningSelection, TargetSelection};
use ndarray::{arr2, Array2};
use tile::Tile;

fn default_tiles() -> Vec<Array2<bool>> {
    vec![
        arr2(&[
            [true, false, false],
            [true, true, true],
            [false, false, true],
        ]),
        arr2(&[[true, false, false, false], [true, true, true, true]]),
        arr2(&[[true, false], [true, true], [true, false], [true, false]]),
        arr2(&[[true, true], [true, true], [true, true]]),
        arr2(&[[true, false], [true, true], [false, true], [false, true]]),
        arr2(&[[true, true], [true, true], [true, false]]),
        arr2(&[[true, true], [true, false], [true, true]]),
        arr2(&[
            [true, false, false],
            [true, false, false],
            [true, true, true],
        ]),
    ]
}

fn default_board_layout() -> Array2<bool> {
    arr2(&[
        [true, true, true, true, true, true, false],
        [true, true, true, true, true, true, false],
        [true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true],
        [true, true, true, false, false, false, false],
    ])
}

fn default_board_meaning_areas() -> Array2<i32> {
    arr2(&[
        [1, 1, 1, 1, 1, 1, -1],
        [1, 1, 1, 1, 1, 1, -1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, -1, -1, -1, -1],
    ])
}

fn default_board_meaning_values() -> Array2<i32> {
    arr2(&[
        [1, 2, 3, 4, 5, 6, -1],
        [7, 8, 9, 10, 11, 12, -1],
        [1, 2, 3, 4, 5, 6, 7],
        [8, 9, 10, 11, 12, 13, 14],
        [15, 16, 17, 18, 19, 20, 21],
        [22, 23, 24, 25, 26, 27, 28],
        [29, 30, 31, -1, -1, -1, -1],
    ])
}

fn default_board_display_values() -> Array2<String> {
    arr2(&[
        ["Jan", "Feb", "Mar", "Apr", "May", "Jun", ""],
        ["Jul", "Aug", "Sep", "Oct", "Nov", "Dec", ""],
        ["1", "2", "3", "4", "5", "6", "7"],
        ["8", "9", "10", "11", "12", "13", "14"],
        ["15", "16", "17", "18", "19", "20", "21"],
        ["22", "23", "24", "25", "26", "27", "28"],
        ["29", "30", "31", "", "", "", ""],
    ])
    .mapv(str::to_string)
}

#[derive(Debug, Clone)]
pub struct DefaultTargetName;
impl TargetName for DefaultTargetName {
    fn create_target_name(&self, selection: &TargetSelection) -> Result<String, String> {
        let meanings = &selection.meaning_selections;
        let slice: &[MeaningSelection; 2] = meanings
            .as_slice()
            .try_into()
            .map_err(|_| "Expected exactly 2 meaning selections".to_string())?;

        let day = slice[0].selected_value;
        let day_name = construct_day_name(day)?;
        let month = slice[1].selected_value;
        let month_name = construct_month_name(month)?;

        Ok(format!("{:02} of {:02}.", day_name, month_name))
    }

    fn box_clone(&self) -> Box<dyn TargetName> {
        Box::new(self.clone())
    }
}

fn year_tiles() -> Vec<Array2<bool>> {
    vec![
        arr2(&[
            [true, false, false],
            [true, true, true],
            [false, false, true],
        ]),
        arr2(&[[true, false, false, false], [true, true, true, true]]),
        arr2(&[[true, false], [true, true], [true, false], [true, false]]),
        arr2(&[[true, true], [true, true], [true, true]]),
        arr2(&[[true, false], [true, true], [false, true], [false, true]]),
        arr2(&[[true, true], [true, true], [true, false]]),
        arr2(&[[true, true], [true, false], [true, true]]),
        arr2(&[
            [true, false, false],
            [true, false, false],
            [true, true, true],
        ]),
        arr2(&[
            [true, true, false],
            [false, true, true],
            [false, true, true],
        ]),
        arr2(&[[true, true, true], [false, true, false]]),
        arr2(&[[true, true, true], [true, false, false]]),
        arr2(&[[true, true, false], [false, true, true]]),
    ]
}

fn year_board_layout() -> Array2<bool> {
    arr2(&[
        [
            true, true, true, true, true, true, false, false, false, false, false,
        ],
        [
            true, true, true, true, true, true, false, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true,
        ],
        [
            true, true, true, false, false, false, false, false, false, false, false,
        ],
    ])
}

fn year_board_meaning_areas() -> Array2<i32> {
    arr2(&[
        [1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1],
        [1, 1, 1, 1, 1, 1, -1, 2, 2, 3, 3],
        [0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3],
        [0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3],
        [0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3],
        [0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3],
        [0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1],
    ])
}

fn year_board_meaning_values() -> Array2<i32> {
    arr2(&[
        [1, 2, 3, 4, 5, 6, -1, -1, -1, -1, -1],
        [7, 8, 9, 10, 11, 12, -1, 1, 2, 1, 2],
        [1, 2, 3, 4, 5, 6, 7, 3, 4, 3, 4],
        [8, 9, 10, 11, 12, 13, 14, 5, 6, 5, 6],
        [15, 16, 17, 18, 19, 20, 21, 7, 8, 7, 8],
        [22, 23, 24, 25, 26, 27, 28, 9, 0, 9, 0],
        [29, 30, 31, -1, -1, -1, -1, -1, -1, -1, -1],
    ])
}

fn year_board_meaning_display_values() -> Array2<String> {
    arr2(&[
        ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "", "", "", "", ""],
        [
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec", "", "1", "2", "1", "2",
        ],
        ["1", "2", "3", "4", "5", "6", "7", "3", "4", "3", "4"],
        ["8", "9", "10", "11", "12", "13", "14", "5", "6", "5", "6"],
        ["15", "16", "17", "18", "19", "20", "21", "7", "8", "7", "8"],
        ["22", "23", "24", "25", "26", "27", "28", "9", "0", "9", "0"],
        ["29", "30", "31", "", "", "", "", "", "", "", ""],
    ])
    .mapv(str::to_string)
}

#[derive(Debug, Clone)]
pub struct YearTargetName;
impl TargetName for YearTargetName {
    fn create_target_name(&self, meaning: &TargetSelection) -> Result<String, String> {
        let meanings = &meaning.meaning_selections;
        let slice: &[MeaningSelection; 4] = meanings
            .as_slice()
            .try_into()
            .map_err(|_| "Expected exactly 4 meaning selections".to_string())?;

        let day = slice[0].selected_value;
        let day_name = construct_day_name(day)?;
        let month = slice[1].selected_value;
        let month_name = construct_month_name(month)?;
        let year_first_digit = slice[2].selected_value;
        let year_second_digit = slice[3].selected_value;

        Ok(format!(
            "{:02} of {:02}. {}{}",
            day_name, month_name, year_first_digit, year_second_digit
        ))
    }

    fn box_clone(&self) -> Box<dyn TargetName> {
        Box::new(self.clone())
    }
}

pub fn get_default_config() -> PuzzleConfig {
    let tiles = create_tiles(&mut default_tiles());
    let board_layout = transform(&mut default_board_layout());
    let meaning_areas = transform(&mut default_board_meaning_areas());
    let meaning_values = transform(&mut default_board_meaning_values());
    let meaning_display_values = transform(&mut default_board_display_values());
    PuzzleConfig::new(
        "Default Puzzle".to_string(),
        board_layout,
        meaning_areas,
        meaning_values,
        meaning_display_values,
        tiles,
        Some(SolutionStatistics {
            min_per_meaning: 7,
            max_per_meaning: 216,
            average_per_meaning: 67.3682795698925,
            mean_per_meaning: 40,
            total_solutions: 25061,
        }),
        vec![
            Meaning {
                index: 0,
                name: "Day".to_string(),
                min: 1,
                max: 31,
            },
            Meaning {
                index: 1,
                name: "Month".to_string(),
                min: 1,
                max: 12,
            },
        ],
        Box::new(DefaultTargetName),
    )
}

pub fn get_year_config() -> PuzzleConfig {
    let tiles = create_tiles(&mut year_tiles());
    let board_layout = transform(&mut year_board_layout());
    let meaning_areas = transform(&mut year_board_meaning_areas());
    let meaning_values = transform(&mut year_board_meaning_values());
    let meaning_display_values = transform(&mut year_board_meaning_display_values());
    PuzzleConfig::new(
        "Year Puzzle".to_string(),
        board_layout,
        meaning_areas,
        meaning_values,
        meaning_display_values,
        tiles,
        Some(SolutionStatistics {
            min_per_meaning: 1292,
            max_per_meaning: 469467,
            average_per_meaning: 37393.1052150538,
            mean_per_meaning: 103348,
            total_solutions: 1391023514,
        }),
        vec![
            Meaning {
                index: 0,
                name: "Day".to_string(),
                min: 1,
                max: 31,
            },
            Meaning {
                index: 1,
                name: "Month".to_string(),
                min: 1,
                max: 12,
            },
            Meaning {
                index: 2,
                name: "First Year Digit".to_string(),
                min: 0,
                max: 9,
            },
            Meaning {
                index: 3,
                name: "Second Year Digit".to_string(),
                min: 0,
                max: 9,
            },
        ],
        Box::new(YearTargetName),
    )
}

fn create_tiles(tile_data_list: &mut Vec<Array2<bool>>) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    for (i, tile_data) in tile_data_list.iter_mut().enumerate() {
        let transformed_data = transform(tile_data);
        tiles.push(Tile::new(i as i32, transformed_data));
    }
    tiles
}

fn construct_day_name(day: i32) -> Result<String, String> {
    match day {
        1 => Ok(format!("{}st", day)),
        2 => Ok(format!("{}nd", day)),
        3 => Ok(format!("{}rd", day)),
        4..=20 | 24..=30 => Ok(format!("{:02}th", day)),
        21 => Ok(format!("{}st", day)),
        22 => Ok(format!("{}nd", day)),
        23 => Ok(format!("{}rd", day)),
        31 => Ok(format!("{}st", day)),
        _ => Err("Day value out of range".to_string()),
    }
}

fn construct_month_name(month: i32) -> Result<String, String> {
    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    if month < 1 || month > 12 {
        return Err("Month value out of range".to_string());
    }
    let month_name = month_names[(month - 1) as usize];
    Ok(month_name.to_string())
}

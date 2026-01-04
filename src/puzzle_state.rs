use crate::offset::CellOffset;
use crate::puzzle::PuzzleConfig;
use gtk::Widget;
use ndarray::Array2;

#[derive(Default, Debug)]
pub struct CellData {
    pub position: CellOffset,
    pub is_on_board: bool,
    pub allowed: bool,
}

#[derive(Debug)]
pub enum Cell {
    Empty(CellData),
    One(CellData, Widget),
    Many(CellData, Vec<Widget>),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty(CellData::default())
    }
}

#[derive(Debug)]
pub struct PuzzleState {
    pub grid: Array2<Cell>,
}

impl PuzzleState {
    pub fn new(puzzle_config: &PuzzleConfig) -> Self {
        let layout = &puzzle_config.board_layout;

        let dim = layout.dim();
        // Add border to have a zone where tiles are not allowed to be placed to indicate out-of-bounds
        let dim = (dim.0 + 2, dim.1 + 2);
        let mut grid: Array2<Cell> = Array2::default(dim);

        for ((r, c), cell) in grid.indexed_iter_mut() {
            let board_index: (i32, i32) = (r as i32 - 1, c as i32 - 1);
            let allowed_on_board = *layout
                .get((board_index.0 as usize, board_index.1 as usize))
                .unwrap_or(&false);
            let allowed =
                allowed_on_board || !Self::is_adjacent_to_allowed(board_index, puzzle_config);
            *cell = Cell::Empty(CellData {
                position: CellOffset(r as i32, c as i32),
                is_on_board: allowed_on_board,
                allowed,
            });
        }

        PuzzleState { grid }
    }

    fn is_adjacent_to_allowed(position: (i32, i32), puzzle_config: &PuzzleConfig) -> bool {
        const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in DELTAS.iter() {
            let neighbor_pos = ((position.0 + dr) as usize, (position.1 + dc) as usize);
            if let Some(allowed) = puzzle_config
                .board_layout
                .get::<(usize, usize)>(neighbor_pos.into())
            {
                if *allowed {
                    return true;
                }
            }
        }
        false
    }
}

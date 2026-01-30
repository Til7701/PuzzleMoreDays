use crate::bitmask::Bitmask;
use crate::tile::Tile;
use banned::BannedBitmask;

mod banned;

pub struct Pruner {
    banned_bitmasks: Vec<BannedBitmask>,
}

impl Pruner {
    /// Creates a new Pruner for use while filling the board with tiles.
    pub fn new_for_filling(board: &crate::board::Board, tiles: &[Tile]) -> Self {
        let banned_bitmasks = banned::create_banned_bitmasks_for_filling(&board, &tiles)
            .into_iter()
            .collect();

        Pruner { banned_bitmasks }
    }

    /// Analyzes the current board state and decides whether a solution is still possible.
    /// If a solution is determined to be impossible, it returns true.
    /// Otherwise, it returns false.
    ///
    /// # Arguments
    ///
    /// * `current_board`: The board to analyze.
    ///
    /// returns: bool
    pub fn prune(&self, current_board: &Bitmask) -> bool {
        for banned in self.banned_bitmasks.iter() {
            if banned.matches(current_board) {
                return true;
            }
        }
        false
    }
}

use crate::tile::Tile;

/// Represents a successful solution to the puzzle.
///
/// TODO placements are not yet filled by the solver.
pub struct Solution {
    pub placements: Vec<TilePlacement>,
}

/// Represents the placement of a tile at a specific position in the puzzle.
pub struct TilePlacement {
    /// The tile being placed.
    pub tile: Tile,
    /// The (x, y) position where the tile is placed.
    pub position: (usize, usize),
}

/// Represents the reason why a puzzle is unsolvable.
///
/// Currently, the only reason is `NoFit`, indicating that no tiles can fit in the remaining spaces.
///
/// In the future, more reasons can be added as needed.
pub enum UnsolvableReason {
    NoFit,
}

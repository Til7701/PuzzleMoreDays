#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    UnsupportedVersion,
    JsonError(String),
    UnknownPredefinedTile { tile_name: String, name: String },
    UnknownCustomBoard { name: String },
    TileWidthOrHeightCannotBeZero { tile_name: String },
    BoardWidthOrHeightCannotBeZero,
}

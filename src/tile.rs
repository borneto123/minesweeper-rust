#[derive(Debug, Clone)]
pub enum Tile {
    Hidden(TileContent),
    Flagged(TileContent),
    Revealed(TileContent)
}
#[derive(Debug, Clone)]
pub enum TileContent {
    Mine,
    Empty(u8),
}

impl Default for Tile {
    fn default() -> Self {
        Self::Hidden(TileContent::Empty(0))
    }
}
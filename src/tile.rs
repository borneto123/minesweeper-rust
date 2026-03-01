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
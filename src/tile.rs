pub enum Tile {
    Hidden(TileContent),
    Flagged(TileContent),
    Revealed(TileContent)
}

pub enum TileContent {
    Mine,
    Empty(u8),
}
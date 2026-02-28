pub struct Tile {
    state: TileState,
}


enum TileState {
    Mine,
    Flag,
    Hidden,
    Revealed,
}
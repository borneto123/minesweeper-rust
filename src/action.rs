use crate::{board::BoardConfig, coords::Coords, dimensions::Dimensions};

pub enum Action {
    StartGame {board_config: BoardConfig},
    FlagTile {coords: Coords},
    RevealTile {coords: Coords}
}


pub enum RevealTileResult {
    MineRevealed,
    TileRevealed(SafeTile)
}

pub enum SafeTile {
    NormalTile,
    LastTile,
}

pub enum ActionError {
    InvalidCoordinate
}
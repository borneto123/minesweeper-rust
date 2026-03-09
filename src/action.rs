use crate::{board::BoardConfig, coords::Coords, dimensions::Dimensions, tile::TileRevealError};

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
    NormalTile(Coords),
    LastTile,
}

pub enum ActionRevealError {
    InvalidCoordinate,
    AlreadyRevealed,
    Flagged,
}

impl From<TileRevealError> for ActionRevealError {
    fn from(value: TileRevealError) -> Self {
        match value {
            TileRevealError::AlreadyRevealed => ActionRevealError::AlreadyRevealed,
            TileRevealError::Flagged => Self::Flagged
        }
    }
}
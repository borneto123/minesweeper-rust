use crate::{board::BoardConfig, coords::Coords, tile::TileRevealError};
#[derive(Debug)]
pub enum Action {
    StartGame {board_config: BoardConfig},
    FlagTile {coords: Coords},
    RevealTile {coords: Coords}
}
#[derive(Debug)]

pub enum RevealTileResult {
    MineRevealed,
    TileRevealed(SafeTile)
}
#[derive(Debug)]

pub enum SafeTile {
    Normal(Coords),
    Flood(Vec<Coords>),
    Last(Coords),
}
#[derive(Debug)]

pub enum ActionRevealError {
    InvalidCoordinate,
    AlreadyRevealed,
    Flagged,
    CoulndtAccesTile,
    Unknown,
}

impl From<TileRevealError> for ActionRevealError {
    fn from(value: TileRevealError) -> Self {
        match value {
            TileRevealError::AlreadyRevealed => ActionRevealError::AlreadyRevealed,
            TileRevealError::Flagged => Self::Flagged
        }
    }
}

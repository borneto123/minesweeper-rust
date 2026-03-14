use crate::{board::config::BoardConfig, coords::Coords, tile::TileRevealError};
#[derive(Debug)]
pub enum Action {
    StartGame {board_config: BoardConfig},
    FlagTile {coords: Coords},
    RevealTile {coords: Coords}
}

pub enum ActionFlagError {

}





use crate::{board::Board, coords::Coords, tile::Tile};

pub struct FlagTileResult {
    new_state: FlagState,
    coords: Coords,
}

pub enum FlagTileError {
    InvalidCoordinate,
    Revealed,
}


pub enum FlagState {
    Flagged,
    UnFlagged,
}

impl Board {

    pub fn toggle_flag(&mut self, coords: &Coords) -> Result<FlagTileResult, FlagTileError> {
        let tile = self
            .get_tile_mut(coords)
            .ok_or(FlagTileError::InvalidCoordinate)?;

        match tile {
            Tile::Revealed(_) => Err(FlagTileError::Revealed),
            Tile::Hidden(_) => {
                tile.flag_tile();

                Ok(FlagTileResult {
                    coords: *coords,
                    new_state: FlagState::Flagged,
                })
            },
            Tile::Flagged(_) => {
                tile.unflag_tile();

                Ok(FlagTileResult {
                    coords: *coords,
                    new_state: FlagState::UnFlagged,
                })
            }

        }

        
    }
}
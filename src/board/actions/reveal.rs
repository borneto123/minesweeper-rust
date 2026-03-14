use crate::{board::Board, coords::Coords, tile::{Tile, TileContent, TileRevealError}};

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
    CouldntAccesTile,
    Unknown,
}

impl Board {
    pub fn reveal_coord(&mut self, coord: &Coords) -> Result<RevealTileResult, ActionRevealError> {
        let tile = self
            .get_tile(&coord)
            .ok_or(ActionRevealError::InvalidCoordinate)?;
        
        let content = tile.content();

        match content {
            TileContent::Mine => {
                Ok(RevealTileResult::MineRevealed)
            },
            TileContent::Empty(0) => {
                let result = self.flood_fill_reveal(coord)?;
                Ok(RevealTileResult::TileRevealed(result))
            },
            TileContent::Empty(_) =>{
                let result = self.single_reveal(coord)?;
                Ok(RevealTileResult::TileRevealed(result))
            }
        }
    }

    fn single_reveal(&mut self, coords: &Coords) -> Result<SafeTile, ActionRevealError> {
        self.hidden_left -= 1; // Function later
        let tile = self
            .get_tile_mut(&coords)
            .ok_or(ActionRevealError::InvalidCoordinate)?;

        tile.reveal_tile()?;
        
        Ok(match  self.no_safe_tiles_left() {
            true => SafeTile::Last(*coords),
            false => SafeTile::Normal(*coords)
        })
    }

    fn flood_fill_reveal(&mut self, start_coord: &Coords) -> Result<SafeTile, ActionRevealError> {
        
        let mut stack  = vec![*start_coord];
        let mut revealed: Vec<Coords> = Vec::new();
        let mut reveal_result : Option<SafeTile> = None;

        while let Some(coord) = stack.pop() {

            let (is_hidden, is_zero) = {
                let tile = self.get_tile(&coord).ok_or(ActionRevealError::Unknown)?;
                (
                    matches!(tile, Tile::Hidden(_)),
                    matches!(tile, Tile::Hidden(TileContent::Empty(0))),
                )
            };

            if is_hidden {
                reveal_result = Some(self.single_reveal(&coord)?);
                revealed.push(coord);
            }

            if is_zero {
                for neighbour in coord.get_neighbours(&self.dimensions) {
                    stack.push(neighbour);
                }
            }
        }
        let reveal_result = reveal_result.ok_or(ActionRevealError::Unknown)?;

        match reveal_result {
            SafeTile::Normal(_) => Ok(SafeTile::Flood(revealed)),
            SafeTile::Last(coord) => Ok(SafeTile::Last(coord)),
            SafeTile::Flood(_) => Err(ActionRevealError::Unknown),
        }
    }
    fn no_safe_tiles_left(&self) -> bool {
        self.hidden_left == 0
    }
 }


 impl From<TileRevealError> for ActionRevealError {
    fn from(value: TileRevealError) -> Self {
        match value {
            TileRevealError::AlreadyRevealed => ActionRevealError::AlreadyRevealed,
            TileRevealError::Flagged => Self::Flagged
        }
    }
}
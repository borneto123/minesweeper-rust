use std::fmt::Display;

use crate::action::{ActionRevealError, RevealTileResult, SafeTile};
use crate::coords::{ Coords};
use crate::tile::{Tile, TileContent};
use crate::dimensions::{Dimensions};
#[derive(Debug)]

pub struct Board {
    tiles : Vec<Tile>,
    dimensions: Dimensions,
    mine_count: i32,
    hidden_left: i32,

}
#[derive(Debug)]

pub struct BoardConfig{
    dimensions: Dimensions,
    mine_count: i32,
}

impl BoardConfig {
    pub fn new(dimensions: Dimensions, mine_count: i32) -> Result<Self, BoardConfigError> {
        if mine_count < 0 {
            return Err(BoardConfigError::IvalidMineCount);
        }

        Ok (BoardConfig{
            dimensions,
            mine_count,
        })
    }
}
#[derive(Debug)]

pub enum BoardError {
    MinePlacementFailed,
    UpdateMineNeighboursFailed,
}
#[derive(Debug)]

pub enum BoardConfigError {
    IvalidMineCount,
}

impl Board {
     
    pub fn new(config: BoardConfig) -> Result<Self, BoardError> {
        let vec_size = config.dimensions.area();
        let tiles = vec![Tile::default(); vec_size as usize];
        let safe_tiles_left = vec_size  - config.mine_count;
        let dimensions = config.dimensions;
        let mine_count = config.mine_count;

        let mut board = 
            Board {
                tiles,
                hidden_left: safe_tiles_left,
                dimensions,
                mine_count,
            };

        board.fill_board()?;
        board.update_mine_neighbours()?;

        Ok(board)
    }

    pub fn tiles (&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tile(&self, coords: &Coords) -> Option<&Tile> {
        let index = coords.to_index(&self.dimensions).ok()?;
        Some(&self.tiles[index as usize])
    }

    fn get_tile_mut(&mut self, coords: &Coords) -> Option<&mut Tile> {
        let index = coords.to_index(&self.dimensions).ok()?;
        Some(&mut self.tiles[index as usize])
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coords, &Tile)> {
        let dim = &self.dimensions;

        self.tiles
            .iter()
            .enumerate()
            .filter_map(move |(i, tile)| {
                let idx = i as i32;
                let coords = Coords::from_index(&idx, &dim).ok()?;
                Some((coords, tile))
            })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Coords, &mut Tile)> {
        let dim = &self.dimensions;

        self.tiles
            .iter_mut()
            .enumerate()
            .filter_map(move |(i, tile)| {
                let idx = i as i32;
                let coords = Coords::from_index(&idx, &dim).ok()?;
                Some((coords, tile))
            })
    }
    pub fn mines_iter(&self) -> impl Iterator<Item = (Coords, &Tile)>{
        self.iter().filter(|(_, tile)| {
            tile.is_mine()
        })
    }

    pub fn mines_iter_mut(&mut self) -> impl Iterator<Item = (Coords, &mut Tile)>{
        self.iter_mut().filter(|(_, tile)| {
            tile.is_mine()
        })
    }

    fn fill_board(&mut self) -> Result<(), BoardError> {
        let mut mine_count = self.mine_count;
        while mine_count !=0 {

            let rand_coords = Coords::new_rand(&self.dimensions);

            let tile = self
                .get_tile_mut(&rand_coords)
                .ok_or(BoardError::MinePlacementFailed)?;

            if tile.place_mine().is_ok() {
                mine_count -= 1;
            }
        }
        Ok(())
    }

    pub fn update_mine_neighbours(&mut self) -> Result<(), BoardError>{

        let mines_coords : Vec<Coords> = self.mines_iter()
            .map(|(coords, _ )|{
                coords
            }).collect();

        for mine_coords in mines_coords {
            for tile_coords in mine_coords.get_neighbours(&self.dimensions){
                let tile = self
                    .get_tile_mut(&tile_coords)
                    .ok_or(BoardError::UpdateMineNeighboursFailed)?;
                if tile.is_mine() {
                    continue;
                }
                tile.increment_empty().map_err(|_| BoardError::UpdateMineNeighboursFailed)?;
            }
        }
        Ok(())
    }

    fn no_safe_tiles_left(&self) -> bool {
        self.hidden_left == 0
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
 }

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_row = 0;
        for (coords, tile) in self.iter() {
            if current_row != coords.row() {
                writeln!(f)?;
                current_row += 1;
            }
            write!(f, "{} ", tile)?;
        }
        writeln!(f)?;
        Ok(())  
    }
}



#[cfg(test)]
mod tests {

    use std::{env::set_current_dir, fmt::Debug};

    use super::*;

    #[test]
    fn new_1() {
        let dim = Dimensions::new(10, 10).ok().unwrap();
        let cfg = BoardConfig::new(dim, 10).ok().unwrap();
        let board = Board::new(cfg)
            .unwrap_or_else(|err| {
                println!("{:?}", err);
                std::process::exit(1);
            });

        for (_, tile) in board.iter() {
            dbg!(tile);
        }
    }
    #[test]
    fn new_2() {
        let dim = Dimensions::new(10, 10).ok().unwrap();
        let cfg = BoardConfig::new(dim, 10).ok().unwrap();
        let mut board = Board::new(cfg)
                .unwrap_or_else(|err| {
                println!("{:?}", err);
                std::process::exit(1);
            });

        println!("{}", board);

        if let Err(err) = board.reveal_coord(&Coords::new(0, 0)) {
            println!("{:?}", err);
        }
        println!("{}", board);
        println!("{}", board.hidden_left);

    }
}
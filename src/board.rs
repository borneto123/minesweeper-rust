use std::collections::VecDeque;
use crate::action::{ActionRevealError, RevealTileResult, SafeTile};
use crate::coords::{ Coords};
use crate::tile::{ Tile, TileContent};
use crate::dimensions::{Dimensions};


pub struct Board {
    tiles : Vec<Tile>,
    dimensions: Dimensions,
    mine_count: i32,
    hidden_left: i32,

}

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

pub enum BoardError {
    MinePlacementFailed,
    UpdateMineNeighboursFailed,
}

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
            true => SafeTile::LastTile,
            false => SafeTile::NormalTile(*coords)
        })
    }

    // fn flood_fill_reveal(&mut self, coords: &Coords) -> SafeTile {
    //     self.single_reveal(coords); // Problem
    //     let dim = &self.dimensions;
    //     let mut empty_neighbours = VecDeque::new();
    //     empty_neighbours.push_back(coords);
        
    //     let empty_neighbours : Vec<Coords>= 
    //         coords
    //             .get_neighbours(dim)
    //             .into_iter()
    //             .filter(|coord| {
    //                 let tile = self.get_tile(coord).unwrap();
    //                 matches!(tile, Tile::Hidden(TileContent::Empty(0)))
    //             })
    //             .collect();        
    //     for coords in empty_neighbours {
    //         self.single_reveal(&coords);
    //     }

        
    //     todo!()
    // }


    pub fn reveal_coord(&mut self, coord: &Coords) -> Result<RevealTileResult, ActionRevealError> {
        let tile = self
            .get_tile(&coord)
            .ok_or(ActionRevealError::InvalidCoordinate)?;
        
        let content = tile.content();

        match content {
            TileContent::Mine => todo!(),
            TileContent::Empty(0) => {
                todo!()
            },
            TileContent::Empty(_) =>{
                let result = self.single_reveal(coord)?;
                Ok(RevealTileResult::TileRevealed(result))
            }
        }
    }
 }


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_1() {
        let dim = Dimensions::new(10, 10).ok().unwrap();
        let cfg = BoardConfig::new(dim, 10).ok().unwrap();
        let board = Board::new(cfg).ok().unwrap();

        for (_, tile) in board.iter() {
            dbg!(tile);
        }
    }
    #[test]
    fn new_2() {
        let dim = Dimensions::new(2, 2).ok().unwrap();
        let cfg = BoardConfig::new(dim, 1).ok().unwrap();
        let board = Board::new(cfg).ok().unwrap();

        for (_, tile) in board.iter() {
            dbg!(tile);
        }
    }
}
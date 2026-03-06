use std::collections::VecDeque;
use std::path::Iter;

use crate::action::{ActionError, RevealTileResult, SafeTile};
use crate::coords::{self, Coords};
use crate::tile::{self, Tile, TileContent};
use crate::dimensions::{Dimensions};


pub struct Board {
    config: BoardConfig,
    tiles : Vec<Tile>,
    hidden: i32,
}

pub struct BoardConfig{
    dimensions: Dimensions,
    mine_count: i32,
}

impl BoardConfig {
    pub fn new(dimensions: Dimensions, mine_count: i32) -> BoardConfig {
        BoardConfig{
            dimensions,
            mine_count,
        }
    }
}


impl Board {
     
    pub fn new(config: BoardConfig) -> Self {
        let vec_size = config.dimensions.area();
        let tiles = vec![Tile::default(); vec_size as usize];
        let safe_tiles_left = vec_size  - config.mine_count;

        let mut board = Board { config, tiles, hidden: safe_tiles_left };

        board.fill_board();
        board.update_mine_neighbours();

        
        board
    }

    pub fn tiles (&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tile(&self, coords: &Coords) -> Option<&Tile> {
        let index = coords.to_index(&self.config.dimensions).ok()?;
        Some(&self.tiles[index as usize])
    }

    pub fn get_tile_mut(&mut self, coords: &Coords) -> Option<&mut Tile> {
        let index = coords.to_index(&self.config.dimensions).ok()?;
        Some(&mut self.tiles[index as usize])
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coords, &Tile)> {
        let dim = &self.config.dimensions;

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
        let dim = &self.config.dimensions;

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

        fn fill_board(&mut self) {
        let mut mine_count = self.config.mine_count;
        while mine_count !=0 {

            let rand_coords = Coords::new_rand(&self.config.dimensions);

            let tile = self.get_tile_mut(&rand_coords).unwrap();

            if tile.place_mine().is_ok() {
                mine_count -= 1;
            }
        }
    }

    pub fn update_mine_neighbours(&mut self) {
        
        let mines_coords : Vec<Coords> = self.mines_iter()
            .map(|(coords, _ )|{
                coords
            }).collect();

        for mine_coords in mines_coords {
            for tile_coords in mine_coords.get_neighbours(&self.config.dimensions){
                let tile = self.get_tile_mut(&tile_coords).unwrap();
                tile.increment_empty();
            }
        }
    }

    fn no_safe_tiles_left(&self) -> bool {
        self.hidden == 0
    }

    fn single_reveal(&mut self, coords: &Coords) -> SafeTile {
        self.hidden -= 1; // Function later
        let tile = self.get_tile_mut(&coords).unwrap();

        tile.reveal_tile();
        
        match  self.no_safe_tiles_left() {
            true => SafeTile::LastTile,
            false => SafeTile::NormalTile
        }
    }

    fn flood_fill_reveal(&mut self, coords: &Coords) -> SafeTile {
        self.single_reveal(coords); // Problem
        let dim = &self.config.dimensions;
        let mut empty_neighbours = VecDeque::new();
        empty_neighbours.push_back(coords);
        
        let empty_neighbours : Vec<Coords>= 
            coords
                .get_neighbours(dim)
                .into_iter()
                .filter(|coord| {
                    let tile = self.get_tile(coord).unwrap();
                    matches!(tile, Tile::Hidden(TileContent::Empty(0)))
                })
                .collect();
        
        
                
        for coords in empty_neighbours {
            self.single_reveal(&coords);
        }

        
        todo!()
    }


    pub fn reveal_coord(&mut self, coord: &Coords) -> Result<RevealTileResult, ActionError> {
        let tile = self.get_tile_mut(&coord).ok_or(ActionError::InvalidCoordinate)?;
        let content = tile.content();

        match content {
            TileContent::Mine => todo!(),
            TileContent::Empty(0) => {
                todo!()
            },
            TileContent::Empty(num) =>{
                Ok(
                    RevealTileResult::TileRevealed(
                        self.single_reveal(coord)
                    )
                )
            }
        }
    }

 }




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_1() {
        let dim = Dimensions::new(10, 10).unwrap();
        let cfg = BoardConfig::new(dim, 10);
        let board = Board::new(cfg);

        for (_, tile) in board.iter() {
            dbg!(tile);
        }
    }
    #[test]
    fn new_2() {
        let dim = Dimensions::new(2, 2).unwrap();
        let cfg = BoardConfig::new(dim, 1);
        let board = Board::new(cfg);

        for (_, tile) in board.iter() {
            dbg!(tile);
        }
    }
}
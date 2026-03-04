use std::path::Iter;

use crate::coords::{self, Coords};
use crate::tile::{self, Tile, TileContent};
use crate::dimensions::{Dimensions};


pub struct Board {
    config: BoardConfig,
    tiles : Vec<Tile>,
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

        let mut board = Board { config, tiles };

        board.fill_board();
        board.update_mine_neighbours();

        board

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

}
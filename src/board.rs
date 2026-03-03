use crate::coords::Coords;
use crate::tile::{Tile, TileContent};
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

/*     pub fn get_neighbours(tile: Tile) -> Vec<Tile> {

    } 
*/
    pub fn tiles (&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn iter(&self) -> BoardIter<'_> {
        BoardIter {
            board: &self,
            index: 0,
        }
    }

    pub fn get_tile(&self, coords: &Coords) -> Option<&Tile> {
        let index = coords.to_index(&self.config.dimensions).ok()?;
        Some(&self.tiles[index as usize])
    }

    pub fn get_tile_mut(&mut self, coords: &Coords) -> Option<&mut Tile> {
        let index = coords.to_index(&self.config.dimensions).ok()?;
        Some(&mut self.tiles[index as usize])
    }

}


pub struct BoardIter< 'a>{
    board: &'a Board,
    index: i32,
}

impl <'a> Iterator for BoardIter<'a> {
    type Item = (Coords, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {

       // Returns None if out of bounds
        let coords = Coords::from_index(&self.index, &self.board.config.dimensions).ok()?;
        let old_index = self.index;
        self.index += 1;
        Some((coords, &self.board.tiles[old_index as usize]))
    }
}


#[cfg(test)]
mod tests {
    use std::ops::SubAssign;

    use super::*;

    #[test]
    fn new() {
        let dim = Dimensions::new(10, 10);
        let cfg = BoardConfig::new(dim, 10);
        let board = Board::new(cfg);

        for (coords, tile) in board.iter() {
            if tile.is_mine() {
            println!("{}, {}",coords.row(), coords.col());
            }
        }
    }

}
use crate::coords::Coords;
use crate::tile::{Tile, TileContent};
use crate::dimensions::Dimensions;


pub struct Board {
    dimensions: Dimensions,
    config: BoardConfig,
    tiles : Vec<Tile>,
}

pub struct BoardConfig{
    dimensions: Dimensions,
    mine_count: u64,
}

impl Board {
    /* 
    pub fn new(config: BoardConfig) -> Self {
        // Filling all tiles with HiddenEmpty(0)
        let tiles = vec![Tile::Hidden(TileContent::Empty(0)); config.dimensions.area()];

    }
*/
    pub fn iter(&self) -> BoardIter<'_> {
        BoardIter {
            board: &self,
            index: 0,
        }
    }

    pub fn get_tile(&self, coords: &Coords) -> Option<&Tile> {
        let index = coords.to_index(&self.dimensions).ok()?;
        Some(&self.tiles[index])
    }
}


pub struct BoardIter< 'a>{
    board: &'a Board,
    index: usize,
}

impl <'a> Iterator for BoardIter<'a> {
    type Item = (Coords, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {

       // Returns None if out of bounds
        let coords = Coords::from_index(&self.index, &self.board.dimensions).ok()?;
        let old_index = self.index;
        self.index += 1;
        Some((coords, &self.board.tiles[old_index]))
    }
}

use crate::coords::Coords;
use crate::tile::Tile;

impl super::Board {
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
}
use crate::{coords::Coords, tile::Tile};

impl super::Board {
    pub fn tiles (&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tile(&self, coords: &Coords) -> Option<&Tile> {
        let index = coords.to_index(&self.dimensions).ok()?;
        Some(&self.tiles[index as usize])
    }

    pub fn get_tile_mut(&mut self, coords: &Coords) -> Option<&mut Tile> {
        let index = coords.to_index(&self.dimensions).ok()?;
        Some(&mut self.tiles[index as usize])
    }
}
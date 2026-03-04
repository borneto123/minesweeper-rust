#[derive(Debug, Clone)]
pub enum Tile {
    Hidden(TileContent),
    Flagged(TileContent),
    Revealed(TileContent)
}
#[derive(Debug, Clone)]
pub enum TileContent {
    Mine,
    Empty(u8),
}
#[derive(Debug, Clone)]

pub enum TileError {
    AlreadyHasMine,
}



impl Tile {
    pub fn is_mine(&self) -> bool {
        matches!(self,
            Tile::Hidden(TileContent::Mine)
            | Tile::Flagged(TileContent::Mine)
            | Tile::Revealed(TileContent::Mine)
        )
    }

    pub fn update(&mut self, new_state: Tile) {
        *self = new_state;
    }

    pub fn set_content(&mut self, new_content: TileContent) {
        match self {
            Tile::Flagged(c)
            | Tile::Revealed(c)
            | Tile::Hidden(c) 
            => *c = new_content
        }
    }

    pub fn place_mine(&mut self) -> Result<(), TileError> {
        if self.is_mine() {
            return  Err(TileError::AlreadyHasMine);
        }

        self.set_content(TileContent::Mine);
        Ok(())
    }
    pub fn increment_empty(&mut self) {
        match self {
            Tile::Flagged(TileContent::Empty(n))
            | Tile::Hidden(TileContent::Empty(n))
            | Tile::Revealed(TileContent::Empty(n)) => {
                *n += 1;
            }
            _ => {}
        }
    }

}


impl Default for Tile {
    fn default() -> Self {
        Self::Hidden(TileContent::Empty(0))
    }
}
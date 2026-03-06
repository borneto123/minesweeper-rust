use rand::distr::slice::Empty;

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
    pub fn content(&self) -> &TileContent {
        match self {
            Tile::Revealed(c)
            | Tile::Hidden(c)
            | Tile::Flagged(c)
            => c
        }
    }

    fn content_mut(&mut self) -> &mut TileContent {
        match self {
            Tile::Revealed(c)
            | Tile::Hidden(c)
            | Tile::Flagged(c)
            => c
        }
    }

    pub fn is_mine(&self) -> bool{
        matches!(self.content(), TileContent::Mine)
    }

    pub fn update(&mut self, new_state: Tile) {
        *self = new_state;
    }

    pub fn set_content(&mut self, new_content: TileContent) {
        *self.content_mut() = new_content;
    }

    pub fn place_mine(&mut self) -> Result<(), TileError> {
        if self.is_mine() {
            return  Err(TileError::AlreadyHasMine);
        }

        self.set_content(TileContent::Mine);
        Ok(())
    }
    pub fn increment_empty(&mut self) {
        if let TileContent::Empty(n) = self.content_mut() {
            *n += 1;
        } 
    }

    pub fn reveal_tile(&mut self) {
        let content = self.content().clone();
        self.update(Tile::Revealed(content));
    }

    pub fn flag_tile(&mut self) {
        let content = self.content().clone();
        self.update(Tile::Flagged(content));
    }

}


impl Default for Tile {
    fn default() -> Self {
        Self::Hidden(TileContent::Empty(0))
    }
}
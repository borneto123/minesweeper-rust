use std::mem::swap;

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

pub enum TilePlaceError {
    AlreadyHasMine,
}

pub enum TileRevealError {
    AlreadyRevealed
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

    pub fn update_state(&mut self, new_state: Tile) {
        *self = new_state;
    }

    pub fn set_content(&mut self, new_content: TileContent) {
        *self.content_mut() = new_content;
    }

    pub fn place_mine(&mut self) -> Result<(), TilePlaceError> {
        if self.is_mine() {
            return Err(TilePlaceError::AlreadyHasMine);
        }
               
        self.set_content(TileContent::Mine);
        Ok(())
    }
    pub fn increment_empty(&mut self) {
        if let TileContent::Empty(n) = self.content_mut() {
            *n += 1;
        } 
    }

    fn take_content(&mut self) -> TileContent {
        let mut tmp = Tile::Hidden(TileContent::Empty(0));
        swap(self, &mut tmp);

        match tmp {
            Tile::Hidden(c)
            | Tile::Revealed(c)
            | Tile::Flagged(c) =>
            c
        }

    }

    pub fn reveal_tile(&mut self) {
        *self = Tile::Revealed(self.take_content());
    }

    pub fn flag_tile(&mut self) {
        *self = Tile::Flagged(self.take_content());
    }

}


impl Default for Tile {
    fn default() -> Self {
        Self::Hidden(TileContent::Empty(0))
    }
}
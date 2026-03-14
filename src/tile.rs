use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Hidden(TileContent),
    Flagged(TileContent),
    Revealed(TileContent)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileContent {
    Mine,
    Empty(u8),
}

#[derive(Debug, Clone)]

pub enum PlaceMineError {
    NotDefault
}

pub enum IncrementEmptyError {
    NotEmpty
}

pub enum TileRevealError {
    AlreadyRevealed,
    Flagged,
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

    fn take_content(&mut self) -> TileContent {
        match std::mem::replace(self, Tile::default()) {
            Tile::Hidden(c)
            | Tile::Revealed(c)
            | Tile::Flagged(c)
            => c,
        }
    }

    pub fn is_mine(&self) -> bool{
        matches!(self.content(), TileContent::Mine)
    }

    pub fn is_revealed(&self) -> bool {
        matches!(self, Tile::Revealed(_))
    }

    pub fn increment_empty(&mut self) -> Result<(), IncrementEmptyError>{
        match self.content_mut() {
            TileContent::Empty(n) => {
                *n += 1;
                Ok(())
            },

            _ =>{Err(IncrementEmptyError::NotEmpty)}
        }
    }

    pub fn place_mine(&mut self) -> Result<(), PlaceMineError> {
        if *self == Tile::default() {
            *self.content_mut() = TileContent::Mine;
            return Ok(())
        }
        Err(PlaceMineError::NotDefault)            
    }

    pub fn reveal_tile(&mut self) -> Result<(), TileRevealError>{
        match self {
            Tile::Revealed(_) => Err(TileRevealError::AlreadyRevealed),
            Tile::Flagged(_) => Err(TileRevealError::Flagged),

            _ => {
                *self = Tile::Revealed(self.take_content());
                Ok(())
            }
        }
    }

    pub fn flag_tile(&mut self) {
        *self = Tile::Flagged(self.take_content());   
    }

    pub fn unflag_tile(&mut self) {
        *self = Tile::Hidden(self.take_content());
    }

}

impl Default for Tile {
    fn default() -> Self {
        Self::Hidden(TileContent::Empty(0))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hidden(_) => write!(f, "#"),
            Self::Flagged(_) => write!(f, "F"),
            Self::Revealed(TileContent::Empty(n)) => write!(f, "{}",n),
            _ => write!(f, "?"),
        }
    }
}
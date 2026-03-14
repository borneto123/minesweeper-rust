mod iter;
pub mod config;
mod generate;
mod access;
mod actions;
mod display;
pub mod error;

use config::BoardConfig;
use crate::board::error::BoardError;
use crate::tile::{Tile};
use crate::dimensions::{Dimensions};


#[derive(Debug)]
pub struct Board {
    tiles : Vec<Tile>,
    dimensions: Dimensions,
    mine_count: i32,
    hidden_left: i32,

}

impl Board {
     pub fn new(config: BoardConfig) -> Result<Self, BoardError> {
        let vec_size = config.dimensions.area();
        let tiles = vec![Tile::default(); vec_size as usize];
        let safe_tiles_left = vec_size  - config.mine_count;
        let dimensions = config.dimensions;
        let mine_count = config.mine_count;

        let mut board = 
            Board {
                tiles,
                hidden_left: safe_tiles_left,
                dimensions,
                mine_count,
            };

        board.fill_board()?;
        board.update_mine_neighbours()?;

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use crate::coords::Coords;

    use super::*;

    #[test]
    fn new_1() {
        let dim = Dimensions::new(10, 10).ok().unwrap();
        let cfg = BoardConfig::new(dim, 10).ok().unwrap();
        let board = Board::new(cfg)
            .unwrap_or_else(|err| {
                println!("{:?}", err);
                std::process::exit(1);
            });

        for (_, tile) in board.iter() {
            dbg!(tile);
        }
    }
    #[test]
    fn new_2() {
        let dim = Dimensions::new(10, 10).ok().unwrap();
        let cfg = BoardConfig::new(dim, 10).ok().unwrap();
        let mut board = Board::new(cfg)
                .unwrap_or_else(|err| {
                println!("{:?}", err);
                std::process::exit(1);
            });

        println!("{}", board);

        if let Err(err) = board.reveal_coord(&Coords::new(0, 0)) {
            println!("{:?}", err);
        }
        println!("{}", board);
        println!("{}", board.hidden_left);

        board.toggle_flag(&Coords::new(1, 1));
        println!("{}", board);
        board.toggle_flag(&Coords::new(1, 1));
        println!("{}", board);
    }
}
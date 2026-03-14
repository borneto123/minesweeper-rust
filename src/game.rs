use std::fmt::Display;

use crate::{board::{Board, actions::{flag::{FlagTileError, FlagTileResult}, reveal::{ActionRevealError, RevealTileResult, SafeTile}}, config::BoardConfig, error::BoardError}, coords::Coords, timer::GameTimer};


pub struct MinesweeperGame {
    timer: GameTimer,
    board: Board,
    outcome: Option<GameResult>,
    started: bool,
}

pub enum GameResult {
    Won,
    Lost,
}

pub enum GameError {
    Board(BoardError),
    Reveal(ActionRevealError),
    Flag(FlagTileError),
    GameEnded,
}

impl MinesweeperGame {
    
    pub fn new(config: BoardConfig) -> Result<Self, GameError>{

        let board = Board::new(config).map_err(|err| GameError::Board(err))?;

        Ok(Self {
            timer: GameTimer::new(),
            board: board,
            outcome: None,
            started: false,
        })
    }

    pub fn reveal_tile(&mut self, coords: &Coords) -> Result<RevealTileResult, GameError> {
        self.start_if_first_move();
        self.action_allowed()?;

        let result = self.board.reveal_coord(coords).map_err(|err| GameError::Reveal(err))?;

        self.update_outcome(&result);
        Ok(result)
    }

    pub fn flag_tile(&mut self, coords: &Coords) -> Result<FlagTileResult, GameError> {
        self.start_if_first_move();
        self.action_allowed()?;

        let result = self.board.toggle_flag(coords).map_err(|err| GameError::Flag(err))?;

        Ok(result)
    }

    pub fn start_if_first_move(&mut self) {
        if self.started == false{
            self.timer.start();
            self.started = true;
        }
    }

    pub fn action_allowed(&self) -> Result<(), GameError>{
        if let Some(_) = self.outcome {
            return  Err(GameError::GameEnded);
        }
        
        Ok(())
    }

    pub fn update_outcome(&mut self, result: &RevealTileResult) {
        match result {
            RevealTileResult::MineRevealed => {
                self.outcome = Some(GameResult::Lost);
                self.timer.end();
            }
            RevealTileResult::TileRevealed(SafeTile::Last(_)) => {
                self.outcome = Some(GameResult::Won);
                self.timer.end();
            },
            _ => ()
        }
    }
}

impl Display for MinesweeperGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}
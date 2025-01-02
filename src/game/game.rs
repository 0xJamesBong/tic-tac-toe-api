//! Tic-Tac-Toe Game

use crate::game::error::Error;
use crate::game::{board::Board, history::History, mark::Mark}; // Import your custom Error type

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Game {
    pub turn: Mark,
    pub board: Board,
    pub history: History,
}

impl Game {
    pub fn new() -> Self {
        Self {
            turn: Mark::X,
            board: Board::new(),
            history: History::new(),
        }
    }
    pub fn mark(&mut self, space: u8) -> Result<(), Error> {
        let space = space.try_into()?;
        self.board.mark(space, self.turn)?;
        self.history.add(space)?;
        self.turn.next();
        Ok(())
    }
}

//! Tic-Tac-Toe Game

use super::*;

use crate::{
    board::Board,
    history::History,
    mark::Mark,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Game {
    turn: Mark,
    board: Board,
    history: History,
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
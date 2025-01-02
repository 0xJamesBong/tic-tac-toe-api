//! Tic-Tac-Toe Errors

use core::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidSpace,
    InvalidMove,
    HistoryFull,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidSpace => write!(f, "invalid space"),
            Error::InvalidMove => write!(f, "invalid move"),
            Error::HistoryFull => write!(f, "game history full"),
        }
    }
}

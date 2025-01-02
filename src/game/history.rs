//! Tic-Tac-Toe History

use crate::game::board::Space;
use crate::game::error::Error;

const MAX_HISTORY_SIZE: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub(crate) struct History((u8, [Option<Space>; MAX_HISTORY_SIZE]));

impl History {
    pub(crate) fn new() -> Self {
        History((0u8, [None; MAX_HISTORY_SIZE]))
    }
    fn len(&self) -> usize {
        self.0 .0 as usize
    }
    pub(crate) fn add(&mut self, space: Space) -> Result<(), Error> {
        if self.0 .0 as usize == MAX_HISTORY_SIZE {
            return Err(Error::HistoryFull);
        };
        self.0 .0 += 1;
        self.0 .1[self.0 .0 as usize] = Some(space);
        Ok(())
    }
}

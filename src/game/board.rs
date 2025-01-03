// ! Tic-Tac-Toe Game Board

extern crate alloc;
use crate::game::error::Error;
use crate::game::mark::Mark;
use core::fmt;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub(crate) struct Space(u8);

impl TryFrom<u8> for Space {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 8 {
            Err(Error::InvalidSpace)
        } else {
            Ok(Space(value))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Board {
    spaces: [Mark; 9],
}

impl Board {
    pub(crate) fn new() -> Self {
        Board {
            spaces: [Mark::Blank; 9],
        }
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let spaces: [Mark; 9] = rng.gen();
        Board { spaces }
    }

    pub(crate) fn mark(&mut self, space: Space, mark: Mark) -> Result<(), Error> {
        self.spaces[space.0 as usize] = mark;
        Ok(())
    }

    pub fn get(&self, space: Space) -> Result<Mark, Error> {
        Ok(self.spaces[space.0 as usize])
    }

    pub fn to_display_array(&self) -> [Option<String>; 9] {
        self.spaces
            .iter()
            .map(|mark| match mark {
                Mark::X => Some("X".to_string()),
                Mark::O => Some("O".to_string()),
                Mark::Blank => None,
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Board should always have 9 spaces")
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [(0, 1, 2), (3, 4, 5), (6, 7, 8)];

        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                writeln!(f, "---+---+---")?;
            }
            writeln!(
                f,
                " {} | {} | {} ",
                self.spaces[line.0], self.spaces[line.1], self.spaces[line.2]
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_board_display() {
        let board_display = alloc::format!("{}", Board::new());
        let expected_display = "   |   |   \n---+---+---\n   |   |   \n---+---+---\n   |   |   \n";
        assert_eq!(board_display, expected_display);
    }

    #[test]
    fn random_boards_not_equal() {
        assert_ne!(Board::random(), Board::random());
    }
}

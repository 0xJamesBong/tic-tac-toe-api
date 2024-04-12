//! Tic-Tac-Toe Marks

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum Mark {
    X = 0x01,
    O = 0x10,
    #[default]
    Blank = 0x00,
}

impl Mark {
    pub(crate) fn next(&mut self) {
        *self = match &self {
            Mark::Blank | Mark::O => Mark::X,
            Mark::X => Mark::O,
        }
    }
}

impl Distribution<Mark> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mark {
        match rng.gen_range(0..=2) {
            0 => Mark::X,
            1 => Mark::O,
            _ => Mark::Blank,
        }
    }
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mark::X => write!(f, "X"),
            Mark::O => write!(f, "O"),
            Mark::Blank => write!(f, " "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn display_x() {
        assert_eq!(format!("{}", Mark::X), "X");
    }

    #[test]
    fn display_o() {
        assert_eq!(format!("{}", Mark::O), "O");
    }

    #[test]
    fn display_blank() {
        assert_eq!(format!("{}", Mark::Blank), " ");
    }

    #[test]
    fn copy() {
        let mark_o = Mark::O;
        let copied_mark_o = mark_o;
        assert_eq!(mark_o, copied_mark_o);
    }

    #[test]
    fn partial_eq() {
        assert_eq!(Mark::X, Mark::X);
        assert_ne!(Mark::X, Mark::O);
    }

    #[test]
    fn default() {
        assert_eq!(Mark::default(), Mark::Blank)
    }
}

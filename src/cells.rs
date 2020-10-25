use termion::color;

#[derive(Copy, Clone)]
/// Represents the states in which a cell can be.
pub enum CellState {
    Mines(u8),
    Covered,
    Flagged,
    MissedMine,
    BadFlag,
}

#[derive(Copy, Clone)]
pub struct Cell {
    /// If the cell has a mine
    pub contains_mine: bool,

    // /// The number of mines there are surrounding this one
    // pub surrounding_mines_count: u8,
    /// In which state should this cell be drawn to the terminal
    pub state: CellState,
}

impl Cell {
    pub const fn new(contains_mine: bool) -> Self {
        Self {
            contains_mine,
            state: CellState::Covered,
        }
    }

    /// If unflagged, flag.
    ///
    /// If flagged, unflag.
    ///
    /// Else, do nothing.
    pub fn toggle_flag(&mut self) {
        match self.state {
            CellState::Flagged => self.state = CellState::Covered,
            CellState::Covered => self.state = CellState::Flagged,
            _ => (),
        }
    }
}

/// Implements Display for the Cell object
/// This makes it so that you can simply print a Cell object
/// and it will display its state with colors and stuff
#[allow(clippy::non_ascii_literal)]
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.state {
            // TODO: Add colors to numbers
            CellState::Mines(n) => match n {
                0 => write!(f, "{}{} ", color::Bg(color::Reset), color::Fg(color::Reset),),
                1 => write!(
                    f,
                    "{}{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::LightBlue),
                    n
                ),
                2 => write!(
                    f,
                    "{}{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::Green),
                    n
                ),
                3 => write!(
                    f,
                    "{}{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::LightRed),
                    n
                ),
                4 => write!(
                    f,
                    "{}{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::Blue),
                    n
                ),
                5 => write!(
                    f,
                    "{}{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::Red),
                    n
                ),
                6 => write!(
                    f,
                    "{}{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::LightCyan),
                    n
                ),
                7 => write!(
                    f,
                    "{}{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::Black),
                    n
                ),
                8 => write!(
                    f,
                    "{}{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::LightBlack),
                    n
                ),
                _ => panic!(),
            },
            CellState::Covered => {
                write!(f, "{}{}▒", color::Bg(color::Reset), color::Fg(color::Reset))
            }
            CellState::Flagged => {
                write!(f, "{}{}◀", color::Bg(color::Reset), color::Fg(color::Red))
            }
            CellState::MissedMine => {
                write!(f, "{}{}X", color::Bg(color::Reset), color::Fg(color::Red))
            }
            CellState::BadFlag => {
                write!(f, "{}{}@", color::Bg(color::Reset), color::Bg(color::Red))
            }
        }
    }
}

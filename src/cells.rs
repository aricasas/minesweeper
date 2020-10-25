use termion::color;

#[derive(Copy, Clone)]
enum CellState {
    Mines(u8),
    Covered,
    Flagged,
    MissedMine,
    BadFlag,
}

#[derive(Copy, Clone)]
pub struct Cell {
    state: CellState,
    pub contains_mine: bool,
}

impl Cell {
    pub const fn new(has_mine: bool) -> Self {
        Self {
            state: CellState::Covered,
            contains_mine: has_mine,
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

    pub fn uncover(&self) {
        // Should check if it has a mine and return something
    }

    pub fn uncover_surroundings(&self) {
        // Get Cell object of the 8 surrounding mines
        // If any are covered, check to see if they have mines
        // Change their states to the correct ones
        // If one has 0 mines, call this function on that cell
    }
}

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

use super::cells;

// (8,8) in minefield
pub const DEFAULT_TERMINAL_POSITION_X: u16 = 17;
pub const DEFAULT_TERMINAL_POSITION_Y: u16 = 12;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Minefield {
    cells: [cells::Cell; 256],
    game_ended: bool,
    remaining_mines: u8,
    cursor_x: u8,
    cursor_y: u8,
}

impl Minefield {
    pub fn new() -> Self {
        Self {
            cells: [cells::Cell::new(false); 256],
            remaining_mines: 30,
            game_ended: false,
            cursor_x: 8,
            cursor_y: 8,
        }
    }

    pub fn restart(&mut self) {
        // Reset variables
        self.cells = [cells::Cell::new(false); 256];
        self.remaining_mines = 30;
        self.game_ended = false;
        self.cursor_x = 8;
        self.cursor_y = 8;
    }

    /// ## Not implemented yet
    ///
    /// Gives the indexes in which to put the 30 mines while making sure that there are
    /// no mines around the protected coordinates. These would be the coordinates of
    /// the first click of the user.
    ///
    /// No index is repeated.
    fn create_new_mine_layout(protected_x: u8, protected_y: u8) -> [usize; 30] {
        [0; 30]
    }

    const fn coordinates_to_index(x: u8, y: u8) -> Option<usize> {
        match (x, y) {
            (1..=16, 1..=16) => Some((((y - 1) * 16) + (x - 1)) as usize),
            _ => None,
        }
    }

    const fn coordinates_to_terminal_coordinates(x: u8, y: u8) -> (u16, u16) {
        // (1,1) -> (3,5)
        // (2,2) -> (5,6)
        // (8,8) -> (17, 12)
        // (16,16) -> (33, 20)
        ((x as u16 * 2) + 1, y as u16 + 4)
    }

    pub fn move_to(&mut self, direction: &Direction) {
        match direction {
            // A bigger y represents down
            Direction::Up => {
                if self.cursor_y > 1 {
                    self.cursor_y -= 1;
                }
            }
            Direction::Down => {
                if self.cursor_y < 16 {
                    self.cursor_y += 1;
                }
            }
            Direction::Right => {
                if self.cursor_x < 16 {
                    self.cursor_x += 1;
                }
            }
            Direction::Left => {
                if self.cursor_x > 1 {
                    self.cursor_x -= 1;
                }
            }
        }
        let (x, y) = Self::coordinates_to_terminal_coordinates(self.cursor_x, self.cursor_y);
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flag(&mut self) {
        // The cursor coordinates should never be out of bounds, so its safe to unwrap()
        let index = Self::coordinates_to_index(self.cursor_x, self.cursor_y).unwrap();

        // We have to be careful to get a reference to it instead of copying
        let cell = &mut self.cells[index];
        cell.toggle_flag();

        let (x, y) = Self::coordinates_to_terminal_coordinates(self.cursor_x, self.cursor_y);
        print!("{}{}", cell, termion::cursor::Goto(x, y));
    }

    pub fn uncover(&mut self) {
        println!("uncover");
    }
}

// Should only be used for the initial print
#[allow(clippy::non_ascii_literal)]
impl std::fmt::Display for Minefield {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "
       \r Remaining mines: 30
       \r q | quit   r | restart
       \r╔═════════════════════════════════╗
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
       \r╚═════════════════════════════════╝
       \r"
        )
    }
}

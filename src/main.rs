#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]

mod cells;
mod minefield;
use minefield::{Direction, Minefield};
use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Get standard input and output
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut minefield = Minefield::new();

    print!(
        "{}{}{}{}{}{}",
        // Clear the screen.
        termion::clear::All,
        // Goto (1,1).
        termion::cursor::Goto(1, 1),
        // Reset colors
        color::Fg(color::Reset),
        color::Bg(color::Reset),
        // Print minefield
        minefield,
        // Goto middle of minefield
        termion::cursor::Goto(
            minefield::DEFAULT_TERMINAL_POSITION_X,
            minefield::DEFAULT_TERMINAL_POSITION_Y
        )
    );

    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for input in stdin.keys() {
        // Print the key we type...
        match input.unwrap() {
            // Break if user pressed q or Ctrl+C
            Key::Char('q') | Key::Ctrl('c') => break,

            Key::Up => minefield.move_to(&Direction::Up),
            Key::Down => minefield.move_to(&Direction::Down),
            Key::Left => minefield.move_to(&Direction::Left),
            Key::Right => minefield.move_to(&Direction::Right),

            // Uncover mine if user pressed Spacebar or Enter
            Key::Char(' ') | Key::Char('\n') => minefield.uncover(),
            // Flag (or unflag) mine if user pressed F key
            Key::Char('f') => minefield.flag(),
            // Restart game if user pressed R key
            Key::Char('r') => {
                // Restart
                minefield.restart();
                print!(
                    "{}{}{}{}{}{}",
                    // Clear the screen.
                    termion::clear::All,
                    // Goto (1,1).
                    termion::cursor::Goto(1, 1),
                    // Reset colors
                    color::Fg(color::Reset),
                    color::Bg(color::Reset),
                    // Print minefield
                    minefield,
                    // Goto middle of minefield
                    termion::cursor::Goto(
                        minefield::DEFAULT_TERMINAL_POSITION_X,
                        minefield::DEFAULT_TERMINAL_POSITION_Y
                    )
                );
            }
            _ => (),
        }

        // Flush again.
        stdout.flush().unwrap();
    }

    print!(
        "{}{}{}",
        // Clear the screen.
        termion::clear::All,
        // Goto (1,1).
        termion::cursor::Goto(1, 1),
        // Show the cursor again before we exit.
        termion::cursor::Show
    );
}

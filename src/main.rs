use std::io::{stdin, stdout, Write};
use termion::{
    clear, color,
    cursor::{/*DetectCursorPos, */ Goto, Hide, Show},
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    terminal_size,
};

macro_rules! clear_screen {
    ($input:expr) => {
        write!($input, "{}", clear::All).unwrap();
    };
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut position = (1, 1);
    let w: u16;
    let h: u16;

    clear_screen!(stdout);

    // get width and height of terminal
    (w, h) = if let Ok((w, h)) = terminal_size() {
        (w, h)
    } else {
        // default to 100x100 terminal
        (100, 100)
    };

    // prepare the terminal
    clear_screen!(stdout);
    write!(stdout, "{}0{}{}", Goto(1, 1), Hide, color::Fg(color::Red)).unwrap();

    // populate the terminal
    for _ in 1..h {
        for _ in 1..w {
            write!(stdout, "#").unwrap();
        }
    }
    writeln!(stdout).unwrap();

    // listen for keys and act on them
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => {
                if 0 < position.0 - 1 {
                    position = (position.0 - 1, position.1);
                }
            }
            Key::Down => {
                if h > position.0 + 1 {
                    position = (position.0 + 1, position.1);
                }
            }
            Key::Left => {
                if 0 < position.1 - 1 {
                    position = (position.0, position.1 - 1);
                }
            }
            Key::Right => {
                if position.1 + 1 < w {
                    position = (position.0, position.1 + 1);
                }
            }
            Key::Esc | Key::Ctrl('c') => break,
            _ => (),
        };

        // prepare the terminal
        stdout.flush().unwrap();
        clear_screen!(stdout);
        write!(stdout, "{}{}", Goto(1, 1), color::Fg(color::Red)).unwrap();

        // update the terminal
        for i in 1..h {
            for j in 1..w {
                if i == position.0 && j == position.1 {
                    write!(
                        stdout,
                        "{}0{}",
                        color::Fg(color::White),
                        color::Fg(color::Red)
                    )
                    .unwrap();
                } else {
                    write!(stdout, "#").unwrap();
                }
            }
        }
        writeln!(stdout).unwrap();

        //(_, h) = stdout.cursor_pos().unwrap();
        //write!(stdout, "{}", Goto(h + 1, 1)).unwrap();
    }

    // return everything back to normal before exiting the program
    stdout.flush().unwrap();
    clear_screen!(stdout);
    write!(stdout, "{}{}", Goto(1, 1), Show).unwrap();
}

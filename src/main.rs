mod menu;

use crate::menu::GameMenu;
use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::{
    clear, color,
    color::Fg,
    cursor::{Goto, Hide, Show},
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

struct Terminal {
    w: u16,
    h: u16,
}

macro_rules! clear_screen {
    ($input:expr) => {
        write!($input, "{}\r", clear::All).unwrap();
    };
}

fn prepare_terminal<C: termion::color::Color>(stdout: &mut RawTerminal<Stdout>, colour: Fg<C>) {
    stdout.flush().unwrap();
    clear_screen!(stdout);
    write!(stdout, "{}{}\r", Goto(1, 1), colour).unwrap();
}

fn update_terminal(stdout: &mut RawTerminal<Stdout>, y: u16, x: u16, h: u16, w: u16) {
    for i in 1..h {
        for j in 1..w {
            if i == y && j == x {
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
        if i != h - 1 {
            write!(stdout, "\n\r").unwrap();
        }
    }
    writeln!(stdout).unwrap();
}

fn new_game(stdout: &mut RawTerminal<Stdout>, h: &mut u16, w: &mut u16) {
    prepare_terminal(stdout, color::Fg(color::White));

    // get width and height of terminal
    (*w, *h) = if let Ok((w, h)) = terminal_size() {
        (w, h)
    } else {
        // default to 100x100 terminal
        (100, 100)
    };

    write!(stdout, "{}0{}{}", Goto(1, 1), Hide, color::Fg(color::Red)).unwrap();

    // populate the terminal
    for i in 1..*h {
        for _ in 1..*w {
            write!(stdout, "#").unwrap();
        }
        if i != *h - 1 {
            write!(stdout, "\n\r").unwrap();
        }
    }
    writeln!(stdout).unwrap();
}

fn update_position(
    c: Result<termion::event::Key, std::io::Error>,
    y: &mut u16,
    x: &mut u16,
    h: u16,
    w: u16,
) -> bool {
    match c.unwrap() {
        Key::Up => {
            if 0 < *y - 1 {
                *y -= 1;
            }
        }
        Key::Down => {
            if h > *y + 1 {
                *y += 1;
            }
        }
        Key::Left => {
            if 0 < *x - 1 {
                *x -= 1;
            }
        }
        Key::Right => {
            if w > *x + 1 {
                *x += 1;
            }
        }
        Key::Esc | Key::Ctrl('c') => return false,
        _ => (),
    };
    true
}

fn game_loop(
    stdin: Stdin,
    stdout: &mut RawTerminal<Stdout>,
    y: &mut u16,
    x: &mut u16,
    h: u16,
    w: u16,
) {
    for c in stdin.keys() {
        if !update_position(c, y, x, h, w) {
            break;
        }

        prepare_terminal(stdout, color::Fg(color::Red));

        update_terminal(stdout, *y, *x, h, w);
    }
}

fn clean_up(stdout: &mut RawTerminal<Stdout>) {
    stdout.flush().unwrap();
    clear_screen!(stdout);
    write!(stdout, "{}{}\r", Goto(1, 1), Show).unwrap();
}

fn main() {
    // locals
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut position = (1, 1);
    let mut terminal = Terminal { w: 100, h: 100 };

    // make a new game
    new_game(&mut stdout, &mut terminal.h, &mut terminal.w);

    // run the game loop
    game_loop(
        stdin,
        &mut stdout,
        &mut position.0,
        &mut position.1,
        terminal.h,
        terminal.w,
    );

    // clean up before exit
    clean_up(&mut stdout);
}

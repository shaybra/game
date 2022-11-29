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

#[derive(Clone, Copy)]
struct Terminal {
    w: u16,
    h: u16,
}

#[derive(Clone, Copy)]
struct Player {
    current_x: u16,
    current_y: u16,
    prev_x: u16,
    prev_y: u16,
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

fn update_terminal(stdout: &mut RawTerminal<Stdout>, player: &mut Player, terminal: Terminal) {
    for i in 1..terminal.h {
        for j in 1..terminal.w {
            if i == player.current_y && j == player.current_x {
                write!(
                    stdout,
                    "{}0{}",
                    color::Fg(color::White),
                    color::Fg(color::Red)
                )
                .unwrap();
            } else if i == player.prev_y && j == player.prev_x {
                write!(stdout, "#").unwrap();
            }
        }
        if i != terminal.h - 1 {
            write!(stdout, "\n\r").unwrap();
        }
    }
    writeln!(stdout).unwrap();
}

fn new_game(stdout: &mut RawTerminal<Stdout>, terminal: &mut Terminal) {
    prepare_terminal(stdout, color::Fg(color::White));

    // get width and height of terminal
    (terminal.w, terminal.h) = if let Ok((w, h)) = terminal_size() {
        (w, h)
    } else {
        // default to 100x100 terminal
        (100, 100)
    };

    write!(stdout, "{}0{}{}", Goto(1, 1), Hide, color::Fg(color::Red)).unwrap();

    // populate the terminal
    for i in 1..terminal.h {
        for _ in 1..terminal.w {
            write!(stdout, "#").unwrap();
        }
        if i != terminal.h - 1 {
            write!(stdout, "\n\r").unwrap();
        }
    }
    writeln!(stdout).unwrap();
}

fn update_position(
    c: Result<termion::event::Key, std::io::Error>,
    player: &mut Player,
    terminal: Terminal,
) -> bool {
    match c.unwrap() {
        Key::Up => {
            if 0 < player.current_y - 1 {
                player.current_y -= 1;
            }
        }
        Key::Down => {
            if terminal.h > player.current_y + 1 {
                player.current_y += 1;
            }
        }
        Key::Left => {
            if 0 < player.current_x - 1 {
                player.current_x -= 1;
            }
        }
        Key::Right => {
            if terminal.w > player.current_x + 1 {
                player.current_x += 1;
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
    player: &mut Player,
    terminal: Terminal
) {
    for c in stdin.keys() {
        player.prev_x = player.current_x;
        player.prev_y = player.current_y;

        if !update_position(c, player, terminal) {
            break;
        }

        prepare_terminal(stdout, color::Fg(color::Red));

        update_terminal(stdout, player, terminal);
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
    let mut terminal = Terminal { w: 100, h: 100 };
    let mut player = Player {
        current_x: 1,
        current_y: 1,
        prev_x: 1,
        prev_y: 1,
    };

    // make a new game
    new_game(&mut stdout, &mut terminal);

    // run the game loop
    game_loop(
        stdin,
        &mut stdout,
        &mut player,
        terminal
    );

    // clean up before exit
    clean_up(&mut stdout);
}

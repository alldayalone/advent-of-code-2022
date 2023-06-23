extern crate termion;
use std::fs;
use std::fmt;


// cool staff
// 1. termion
// 2. /r/n https://stackoverflow.com/questions/48494508/how-do-i-create-a-new-line-when-using-termion-in-raw-mode

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use std::io::{Write, stdout, stdin};
struct GameState {
    head_i: i32,
    head_j: i32,
    tail_i: i32,
    tail_j: i32,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            head_i: 0,
            head_j: 0,
            tail_i: 0,
            tail_j: 0,
        }
    }

    fn move_top(&mut self) {
        self.head_i += 1;

        if self.tail_i <= self.head_i - 2 {
            self.tail_i = self.head_i - 1;
            self.tail_j = self.head_j;
        }
    }

    fn move_right(&mut self) {
        self.head_j += 1;

        if self.tail_j <= self.head_j - 2 {
            self.tail_j = self.head_j - 1;
            self.tail_i = self.head_i;
        }
    }

    fn move_down(&mut self) {
        self.head_i -= 1;

        if self.tail_i >= self.head_i + 2 {
            self.tail_i = self.head_i + 1;
            self.tail_j = self.head_j;
        }
    }

    fn move_left(&mut self) {
        self.head_j -= 1;

        if self.tail_j >= self.head_j + 2 {
            self.tail_j = self.head_j + 1;
            self.tail_i = self.head_i;
        }
    }
}

impl fmt::Display for GameState {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        for i in (0..10).rev() {
            for j in 0..10 {
                if i == self.head_i && j == self.head_j {
                    write!(f, "H")?;
                } else if i == self.tail_i && j == self.tail_j {
                    write!(f, "T")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\r\n")?;
        }
        Result::Ok(())
    }
}

fn main() { 
    let input = fs::read_to_string("src/input9_test.txt").unwrap();

    let mut game_state = GameState::new();
    
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    print!("{}", game_state);

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('d')) =>  {
                game_state.move_right();
            },
            Event::Key(Key::Char('a')) =>  {
                game_state.move_left();
            },
            Event::Key(Key::Char('w')) =>  {
                game_state.move_top();
            },
            Event::Key(Key::Char('s')) =>  {
                game_state.move_down();
            },
            _ => {}
        }
        write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        print!("{}", game_state);
    }
}
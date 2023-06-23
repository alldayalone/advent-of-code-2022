extern crate termion;
use std::fs;
use std::fmt;
use std::collections::HashSet;
use std::{thread, time};

// cool staff
// 1. termion
// 2. /r/n https://stackoverflow.com/questions/48494508/how-do-i-create-a-new-line-when-using-termion-in-raw-mode
// 3. sleep


use termion::event::{Key, Event};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
struct GameState {
    start: (i32, i32),
    knots: Vec<(i32, i32)>,
    tail_history: HashSet<(i32, i32)>,
}

impl GameState {
    fn new(start: (i32, i32), size: usize) -> GameState {
        GameState {
            start,
            knots: vec![start; size],
            tail_history: HashSet::from([start]),
        }
    }

    fn move_top(&mut self) {
        self.knots[0].0 += 1;

        self.move_tail();
    }

    fn move_right(&mut self) {
        self.knots[0].1 += 1;

        self.move_tail();
    }

    fn move_down(&mut self) {
        self.knots[0].0 -= 1;

        self.move_tail();
    }

    fn move_left(&mut self) {
        self.knots[0].1 -= 1;

        self.move_tail();
    }

    fn move_tail(&mut self) {
        for i in 1..self.knots.len() {
            if self.knots[i].0 <= self.knots[i - 1].0 - 2 {
                self.knots[i].0 += 1;
                self.knots[i].1 += (self.knots[i - 1].1 - self.knots[i].1).signum();
            }

            if self.knots[i].1 <= self.knots[i - 1].1 - 2 {
                self.knots[i].1 += 1;
                self.knots[i].0 += (self.knots[i - 1].0 - self.knots[i].0).signum();
            }

            if self.knots[i].0 >= self.knots[i - 1].0 + 2 {
                self.knots[i].0 -= 1;
                self.knots[i].1 += (self.knots[i - 1].1 - self.knots[i].1).signum();
            }

            if self.knots[i].1 >= self.knots[i - 1].1 + 2 {
                self.knots[i].1 -= 1;
                self.knots[i].0 += (self.knots[i - 1].0 - self.knots[i].0).signum();
            }
        }

        self.tail_history.insert(self.knots.last().unwrap().to_owned());
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..26).rev() {
            for j in 0..26 {
                if self.knots[0] == (i, j) {
                    write!(f, "H")?;
                    continue;
                }
                let pos = self.knots.iter().skip(1).position(|knot| knot == &(i, j));

                if pos.is_some() {
                    write!(f, "{}", pos.unwrap() + 1)?;
                    continue;
                }
                
                if self.tail_history.contains(&(i, j)) {
                    write!(f, "#")?;
                } else if self.start == (i, j) {
                    write!(f, "s")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\r\n")?;
        }
        write!(f, "Tail moves: {}\r\n", self.tail_history.len())?;
        Result::Ok(())
    }
}

fn main() { 

    let mut game_state = GameState::new((5, 11), 10);
    
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    print!("{}", game_state);

    let input = fs::read_to_string("src/input9_test2.txt").unwrap();
    input.lines().for_each(|line| {
        let (direction, distance) = line.split_at(1);
        let distance = distance.trim().parse::<i32>().unwrap();

        for _ in 0..distance {
            match direction {
                "U" => {
                    game_state.move_top();
                },
                "R" => {
                    game_state.move_right();
                },
                "D" => {
                    game_state.move_down();
                },
                "L" => {
                    game_state.move_left();
                },
                _ => {}
            }
        }

        write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        print!("{}", game_state);
        thread::sleep(time::Duration::from_millis(5000));
    });

    write!(stdout, "{}{}Use wasd to move head around. q to exit. r to reset\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
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
            Event::Key(Key::Char('r')) =>  {
                game_state = GameState::new((5, 11), 10);
            },
            _ => {}
        }
        write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        print!("{}", game_state);
    }
}
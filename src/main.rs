use std::fs;
use regex::Regex;
extern crate termion;

use termion::event::{Key, Event};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

#[derive(Eq, PartialEq, Debug,Clone)]
enum Facing {
  Up,
  Right,
  Down,
  Left,
}

const TILE: usize = 3;
const TILE2: usize = 6;
const TILE3: usize = 9;
const TILE4: usize = 12;

const TILEA: (usize, usize) = (TILE, 0);
const TILEB: (usize, usize) = (TILE2, 0);
const TILEC: (usize, usize) = (TILE, TILE);
const TILED: (usize, usize) = (0, TILE2);
const TILEE: (usize, usize) = (TILE, TILE2);
const TILEF: (usize, usize) = (0, TILE3);

fn map(el: usize, from: usize, to: usize, flip: bool) -> usize {
  let delta = el - from;

  if flip {
    to + (TILE - 1 - delta)
  } else {
    to + delta
  }
}

fn parse_instructions(line: &str) -> Vec<&str> {
  Regex::new(r"(\d+)([RL])?").unwrap()
    .captures_iter(line)
    .flat_map(|caps| 
      caps
        .iter()
        .skip(1)
        .filter_map(|m| m)
        .map(|m| m.as_str())
        .collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

fn display_field(field: &Vec<Vec<char>>, x: usize, y: usize, facing: &Facing) {
    print!("==================\r\n");
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if j == x && i == y {
                print!("{}", match facing {
                    Facing::Up => "^",
                    Facing::Right => ">",
                    Facing::Down => "v",
                    Facing::Left => "<",
                });
            } else {
                print!("{}", field[i][j]);
            }
            // print!("{}", if rock field[i][j] { '#' } else { '.' });
        }
        print!("\r\n");
        // println!();
    }
}

fn main() {
  let input = fs::read_to_string("src/input22_three.txt").unwrap();

  let mut field = input.lines().take_while(|line| !line.is_empty()).map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
  let instructions = parse_instructions(input.lines().last().unwrap());

  println!("Field: {:?}", field);

  let x_bounds = field.iter().map(|row| 
    (
      row.iter().position(|c| c == &'.' || c == &'#').unwrap(),
      row.iter().rposition(|c| c == &'.' || c == &'#').unwrap()
    )
  ).collect::<Vec<_>>();
  let y_bounds = (0..field.iter().map(|r| r.len()).max().unwrap()).map(|i| {
    (
      field.iter().position(|row| row.get(i).is_some_and(|c| c == &'.' || c == &'#')).unwrap(),
      field.iter().rposition(|row| row.get(i).is_some_and(|c| c == &'.' || c == &'#')).unwrap()
    )
  }).collect::<Vec<_>>();

  let mut facing = Facing::Right;
  let mut x = x_bounds[0].0;
  let mut y = 0;

  let stdin = stdin();
  let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    display_field(&field, x, y, &facing);
  
  /** this is main */
  // for instr in instructions {
    // println!("Row: {}, column: {}, x: {}, y: {}, facing: {:?}, instr: {}", y + 1, x + 1, x, y, facing, instr);
    // match instr {
    //   "R" => {
    //     facing = match facing {
    //       Facing::Up => Facing::Right,
    //       Facing::Right => Facing::Down,
    //       Facing::Down => Facing::Left,
    //       Facing::Left => Facing::Up,
    //     }
    //   },
    //   "L" => {
    //     facing = match facing {
    //       Facing::Up => Facing::Left,
    //       Facing::Right => Facing::Up,
    //       Facing::Down => Facing::Right,
    //       Facing::Left => Facing::Down,
    //     }
    //   },
    //   _ => {
    //     let steps = instr.parse::<usize>().unwrap();
    //     for _ in 0..steps {

    for c in stdin.events() {
      let evt = c.unwrap();
      let mut is_turn = false;
      match evt {
        Event::Key(Key::Char('a')) => {
          if facing != Facing::Left {
            facing = Facing::Left;
            is_turn = true;
          }
        },
        Event::Key(Key::Char('w')) => {
          if facing != Facing::Up {
            facing = Facing::Up;
            is_turn = true;
          }
        },
        Event::Key(Key::Char('s')) => {
          if facing != Facing::Down {
            facing = Facing::Down;
            is_turn = true;
          }
        },
        Event::Key(Key::Char('d')) => {
          if facing != Facing::Right {
            facing = Facing::Right;
            is_turn = true;
          }
        },
        _ => {}
      }

      if is_turn {
        write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        display_field(&field, x, y, &facing);
        continue;
      }

      match evt {
        Event::Key(Key::Char('a')) | Event::Key(Key::Char('w')) | Event::Key(Key::Char('s')) | Event::Key(Key::Char('d')) => {
          let mut new_x = x;
          let mut new_y = y;
          let mut new_facing = facing.clone();

          // Tile A
          if (TILEA.0..TILEA.0+TILE).contains(&x) && y == TILEA.1 && facing == Facing::Up {
            new_x = TILEF.0;
            new_y = map(x, TILEA.0, TILEF.1, false);
            new_facing = Facing::Right;
          } else if x == TILEA.0 && (TILEA.1..TILEA.1 + TILE).contains(&y) && facing == Facing::Left {
            new_x = TILED.0;
            new_y = map(y, TILEA.1, TILED.1, true);
            new_facing = Facing::Right;
          }
          // Tile B          
          else if (TILEB.0..TILEB.0+TILE).contains(&x) && y == TILEB.1 && facing == Facing::Up {
            new_x = map(x, TILEB.0, TILEF.0, false);
            new_y = TILEF.1 + TILE - 1;
          } else if x == TILEB.0 + TILE - 1 && (TILEB.1..TILEB.1 + TILE).contains(&y) && facing == Facing::Right {
            new_x = TILEE.0 + TILE - 1;
            new_y = map(y, TILEB.1, TILEE.1, true);
            new_facing = Facing::Left;
          } else if (TILEB.0..TILEB.0 + TILE).contains(&x) && y == TILEB.1 + TILE - 1 && facing == Facing::Down {
            new_x = TILEC.0 + TILE - 1;
            new_y = map(x, TILEB.0, TILEC.1, false);
            new_facing = Facing::Left;
          }
          
          // Tile C
          else if x == TILEC.0 + TILE - 1 && (TILEC.1..TILEC.1 + TILE).contains(&y) && facing == Facing::Right {
            new_x = map(y, TILEC.1, TILEB.0, false);
            new_y = TILEB.1 + TILE - 1;
            new_facing = Facing::Up;
          } else if x == TILEC.0 && (TILEC.1..TILEC.1 + TILE).contains(&y) && facing == Facing::Left {
            new_x = map(y, TILEC.1, TILED.0, false);
            new_y = TILED.1;
            new_facing = Facing::Down;
          }

          // Tile D          
          else if (TILED.0..TILED.0 + TILE).contains(&x) && TILED.1 == y && facing == Facing::Up {
            new_x = TILEC.0;
            new_y = map(x, TILED.0, TILEC.1, false);
            new_facing = Facing::Right;
          } else if x == TILED.0 && (TILED.1..TILED.1 + TILE).contains(&y) && facing == Facing::Left {
            new_x = TILEA.0;
            new_y = map(y, TILED.1, TILEA.1, true);
            new_facing = Facing::Right;
          }
          
          // Tile E
          else if x == TILEE.0 + TILE - 1 && (TILEE.1..TILEE.1 + TILE).contains(&y) && facing == Facing::Right {
            new_x = TILEB.0 + TILE - 1;
            new_y = map(y, TILEE.1, TILEB.1, true);
            new_facing = Facing::Left;
          } else if (TILEE.0..TILEE.0 + TILE).contains(&x) && TILEE.1 + TILE - 1 == y && facing == Facing::Down {
            new_x = TILEF.0 + TILE - 1;
            new_y = map(x, TILEE.0, TILEF.1, false);
            new_facing = Facing::Left;
          }

          // Tile F
          else if x == TILEF.0 + TILE - 1 && (TILEF.1..TILEF.1 + TILE).contains(&y) && facing == Facing::Right {
            new_x = map(y, TILEF.1, TILEE.0, false);
            new_y = TILEE.1 + TILE - 1;
            new_facing = Facing::Up;
          } else if (TILEF.0..TILEF.0 + TILE).contains(&x) && TILEF.1 + TILE - 1 == y && facing == Facing::Down {
            new_x = map(x, TILEF.0, TILEB.0, false);
            new_y = TILEB.1;
          } else if x == TILEF.0 && (TILEF.1..TILEF.1 + TILE).contains(&y) && facing == Facing::Left {
            new_x = map(y, TILEF.1, TILEA.0, false);
            new_y = TILEA.1;
            new_facing = Facing::Down;
          }
          
          // Restw
          else {
            new_x = match facing {
              Facing::Right => if x >= x_bounds[y].1 { x_bounds[y].0 } else { x + 1 },
              Facing::Left =>if x <= x_bounds[y].0 { x_bounds[y].1 } else { x - 1 },
              _ => x,
            };
            new_y = match facing {
              Facing::Up => if y <= y_bounds[x].0 { y_bounds[x].1 } else { y - 1 },
              Facing::Down => if y >= y_bounds[x].1 { y_bounds[x].0 } else { y + 1 },
              _ => y,
            };
          }

          if field[new_y][new_x] == '#' {
            // println!("found wall at x: {}, y: {}", new_x, new_y);
            // break;
            continue;
          }

          x = new_x;
          y = new_y;
          facing = new_facing;
        },
      _ => {}
    }
    write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    display_field(&field, x, y, &facing);
  }

  //     }
  //   }
  // }

  let facing_val = match facing {
    Facing::Right => 0,
    Facing::Down => 1,
    Facing::Left => 2,
    Facing::Up => 3,
  };
  let answer = 1000*(y + 1) + 4*(x + 1) + facing_val;

  println!("answer: {}, Row: {}, column: {}, x: {}, y: {}, facing: {:?}", answer, y + 1, x + 1, x, y, facing);
}
use std::fs;

extern crate termion;
// use termion::event::{Key, Event};
// use termion::input::{TermRead, MouseTerminal};
// use termion::raw::IntoRawMode;
// use std::io::{Write, stdout, stdin};

const FIELD_WIDTH: usize = 7;
const FIELD_HEIGHT: usize = 10000;

const ROCK_WIDTH: usize = 4;
const ROCK_HEIGHT: usize = 4;

const ITERATIONS: usize = 2022;

#[derive(Clone, Copy)]
struct Position {
    left: usize,
    bot: usize
}

// static mut MSG: String = String::new();

type RockMask = [[bool; ROCK_WIDTH]; ROCK_HEIGHT];
type Field = [[bool; FIELD_WIDTH]; FIELD_HEIGHT];

// fn display_field(field: &Field, position: &Position, rock: &Rock, height: usize) {
//     print!("==================\r\n");
//     for i in (0..height + 4).rev() {
//         for j in 0..FIELD_WIDTH {
//             if j >= position.left && (position.bot + ROCK_HEIGHT - 1).checked_sub(i).is_some_and(|index| rock.mask.get(index).is_some_and(|row| row.get(j - position.left).is_some_and(|&cell| cell))) {
//                 print!("@");
//             } else {
//                 print!("{}", if field[i][j] { '#' } else { '.' });
//             }
//             // print!("{}", if rock field[i][j] { '#' } else { '.' });
//         }
//         print!("\r\n");
//         // println!();
//     }
// }

fn display_field_without_rock(field: &Field, height: usize) {
    print!("==================\r\n");
    for i in (0..height + 4).rev() {
        for j in 0..FIELD_WIDTH {
            print!("{}", if field[i][j] { '#' } else { '.' });
        }
        println!();
    }
}



struct Rock {
    mask: RockMask,
    height: usize,
    width: usize
}

const DASH: Rock = Rock {
    mask: [[false,false,false,false],[false,false,false,false],[false,false,false,false],[true,true,true,true]],
    height: 1,
    width: 4
};

const PLUS: Rock = Rock {
    mask: [[false,false,false,false],[false,true,false,false],[true,true,true,false],[false,true,false,false]],
    height: 3,
    width: 3
};

const ANGLE: Rock = Rock {
    mask: [[false,false,false,false],[false,false,true,false],[false,false,true,false],[true,true,true,false]],
    height: 3,
    width: 3
};

const VLINE: Rock = Rock {
    mask: [[true,false,false,false],[true,false,false,false],[true,false,false,false],[true,false,false,false]],
    height: 4,
    width: 1
};

const SQUARE: Rock = Rock {
    mask: [[false,false,false,false],[false,false,false,false],[true,true,false,false],[true,true,false,false]],
    height: 2,
    width: 2
};
const ROCKS: [Rock; 5] = [DASH, PLUS, ANGLE, VLINE, SQUARE];

impl Rock {
    // pub const DASH: u16 = 0b0000_0000_0000_1111;
    // pub const PLUS: u16 = 0b0000_0100_1110_0100;
    // pub const ANGLE: u16 = 0b0000_0010_0010_1110;
    // pub const VLINE: u16 = 0b1000_1000_1000_1000;
    // pub const SQUARE: u16 = 0b0000_0000_1100_1100;

    // const DASH: RockMask = [[false,false,false,false],[false,false,false,false],[false,false,false,false],[true,true,true,true]];
    // const PLUS: RockMask = [[false,false,false,false],[false,true,false,false],[true,true,true,false],[false,true,false,true]];
    // const ANGLE: RockMask = [[false,false,false,false],[false,false,true,false],[false,false,true,false],[true,true,true,false]];
    // const VLINE: RockMask = [[true,false,false,false],[true,false,false,false],[true,false,false,false],[true,false,false,false]];
    // const SQUARE: RockMask = [[false,false,false,false],[false,false,false,false],[true,true,false,false],[true,true,false,false]];
    fn move_left(field: &Field, rock: &RockMask, position: &Position) -> Position {
        for i in 0..ROCK_HEIGHT {
            for j in 0..ROCK_WIDTH {
                if position.left == 0 || field.get(position.bot + i).is_some_and(|row| row.get(position.left + j - 1).is_some_and(|&cell| cell && rock[ROCK_HEIGHT - i - 1][j])) {
                    return position.clone();
                }
            }
        }

        Position { left: position.left - 1, bot: position.bot }
    }

    fn move_right(field: &Field, rock: &RockMask, position: &Position) -> Position {
        for i in 0..ROCK_HEIGHT {
            for j in 0..ROCK_WIDTH {
                let cell = field.get(position.bot + i).map(|row| row.get(position.left + j + 1)).flatten();

                if rock[ROCK_HEIGHT - i - 1][j] && (cell.is_none() || cell.is_some_and(|&cell| cell)) {
                    return position.clone();
                }
            }
        }

        Position { left: position.left + 1, bot: position.bot }
    }

    fn move_down(field: &Field, rock: &RockMask, position: &Position) -> (Position, bool) {
        for i in 0..ROCK_HEIGHT {
            for j in 0..ROCK_WIDTH {
                if position.bot == 0 || field.get(position.bot + i - 1).is_some_and(|row| row.get(position.left + j).is_some_and(|&cell| cell && rock[ROCK_HEIGHT - i - 1][j])) {
                    // unsafe { MSG = format!("{} {}; pos {} {} ", i, j, position.left, position.bot); }
                    return (position.clone(), true);
                }
            }
        }

        (Position { left: position.left, bot: position.bot - 1 }, false)
    }

    fn freeze(field: &mut Field, rock: &Rock, position: &Position) {
        for i in 0..ROCK_HEIGHT {
            for j in 0..rock.width {
                field[position.bot + i][position.left + j] = field[position.bot + i][position.left + j] || rock.mask[ROCK_HEIGHT - i - 1][j];
            }
        }
    }
}

fn main() { 
    let input = fs::read_to_string("src/input17.txt").expect("Input file exists");
    let mut jet_pattern = input.chars().into_iter().cycle();
    let mut rocks = ROCKS.iter().cycle();
    let mut field: Field = [[false; FIELD_WIDTH]; FIELD_HEIGHT];
    let mut high_point = 0;

   
    
    // let mut rock = rocks.next().unwrap();
    // let mut position = Position { left: 2, bot: high_point + 3 };
    
    // let stdin = stdin();
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    // write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    // display_field(&field, &position, &rock, high_point);

    // for c in stdin.events() {
    //     let evt = c.unwrap();
    //     match evt {
    //         Event::Key(Key::Char('q')) => break,
    //         Event::Key(Key::Char(' ')) => {
    //             Rock::freeze(&mut field, &rock, &position);

    //             high_point = high_point.max(position.bot + rock.height);
    //             rock = rocks.next().unwrap();
    //             position = Position { left: 2, bot: high_point + 3 };
    //         },
    //         Event::Key(Key::Down) => {
    //             (position, _) = Rock::move_down(&field, &rock.mask, &position);
    //         },
    //         Event::Key(Key::Right) => {
    //             position = Rock::move_right(&field, &rock.mask, &position);
    //         },
    //         Event::Key(Key::Left) => {
    //             position = Rock::move_left(&field, &rock.mask, &position);
    //         },
    //         _ => {}
    //     }
    //     write!(stdout, "{}{}Use wasd to move head around. q to exit\r\n{}\r\n", termion::clear::All, termion::cursor::Goto(1, 1), unsafe { MSG.clone() }).unwrap();
    //     display_field(&field, &position, &rock, high_point);

    // }

    'next_rock: for _ in 0..ITERATIONS {
        let rock = rocks.next().unwrap();
        
        let mut position = Position { left: 2, bot: high_point + 3 };

        loop {
            match jet_pattern.next().unwrap() {
                '<' => position = Rock::move_left(&field, &rock.mask, &position),
                '>' => position = Rock::move_right(&field, &rock.mask, &position),
                _ => unreachable!("Invalid input")
            }

            let is_stuck;

            (position, is_stuck) = Rock::move_down(&field, &rock.mask, &position);

            if is_stuck {
                Rock::freeze(&mut field, &rock, &position);

                high_point = high_point.max(position.bot + rock.height);

                continue 'next_rock;
            }
        }
    } 

    display_field_without_rock(&field, high_point);
    println!("High point: {}", high_point);
}
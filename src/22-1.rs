use std::fs;
use regex::Regex;

#[derive(Debug,Clone)]
enum Facing {
  Up,
  Right,
  Down,
  Left,
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

fn main() {
  let input = fs::read_to_string("src/input22.txt").unwrap();

  let field = input.lines().take_while(|line| !line.is_empty()).map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
  let instructions = parse_instructions(input.lines().last().unwrap());

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

  for instr in instructions {
    // println!("Row: {}, column: {}, x: {}, y: {}, facing: {:?}, instr: {}", y + 1, x + 1, x, y, facing, instr);
    match instr {
      "R" => {
        facing = match facing {
          Facing::Up => Facing::Right,
          Facing::Right => Facing::Down,
          Facing::Down => Facing::Left,
          Facing::Left => Facing::Up,
        }
      },
      "L" => {
        facing = match facing {
          Facing::Up => Facing::Left,
          Facing::Right => Facing::Up,
          Facing::Down => Facing::Right,
          Facing::Left => Facing::Down,
        }
      },
      _ => {
        let steps = instr.parse::<usize>().unwrap();
        for _ in 0..steps {
          let new_x = match facing {
            Facing::Right => if x >= x_bounds[y].1 { x_bounds[y].0 } else { x + 1 },
            Facing::Left =>if x <= x_bounds[y].0 { x_bounds[y].1 } else { x - 1 },
            _ => x,
          };
          let new_y = match facing {
            Facing::Up => if y <= y_bounds[x].0 { y_bounds[x].1 } else { y - 1 },
            Facing::Down => if y >= y_bounds[x].1 { y_bounds[x].0 } else { y + 1 },
            _ => y,
          };

          if field[new_y][new_x] == '#' {
            // println!("found wall at x: {}, y: {}", new_x, new_y);
            break;
          }

          x = new_x;
          y = new_y;
        }
      }
    }
  }

  let facing_val = match facing {
    Facing::Right => 0,
    Facing::Down => 1,
    Facing::Left => 2,
    Facing::Up => 3,
  };
  let answer = 1000*(y + 1) + 4*(x + 1) + facing_val;

  println!("answer: {}, Row: {}, column: {}, x: {}, y: {}, facing: {:?}", answer, y + 1, x + 1, x, y, facing);
}
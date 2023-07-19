use std::fmt::{Display, Formatter};
use std::fs;
use std::collections::{VecDeque, HashMap};

struct Field {
  data: VecDeque<VecDeque<char>>,
}

impl Display for Field {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for row in self.data.iter() {
      for col in row.iter() {
        write!(f, "{}", col)?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

impl Field {
  fn new(input: String) -> Field {
    let data: VecDeque<VecDeque<char>> = input.lines().map(|line| line.chars().collect()).collect();

    Field { data }
  }

  // If any elf '#' is on the edge, add a frame of '.' to the field
  fn expand_if_needed(&mut self) {
    for y in 0..self.data.len() {
      for x in 0..self.data[y].len() {
        if self.data[y][x] == '#' && (x == 0 || x == self.data[y].len() - 1 || y == 0 || y == self.data.len() - 1) {
          // add frame
          for row in self.data.iter_mut() {
            row.push_front('.');
            row.push_back('.');
          }
          let mut frame = VecDeque::new();
          for _ in 0..self.data[0].len() {
            frame.push_back('.');
          }
          self.data.push_front(frame.clone());
          self.data.push_back(frame);
          break;
        }
      }
    }
  }

  fn is_square_taken(&self, position: Position) -> bool {
    self.data[position.y][position.x] == '#'
  }

  fn is_north_taken(&self, position: &Position) -> bool {
    self.is_square_taken(Position { x: position.x - 1, y: position.y - 1 }) ||
    self.is_square_taken(Position { x: position.x, y: position.y - 1 }) ||
    self.is_square_taken(Position { x: position.x + 1, y: position.y - 1 })
  }

  fn is_south_taken(&self, position: &Position) -> bool {
    self.is_square_taken(Position { x: position.x - 1, y: position.y + 1 }) ||
    self.is_square_taken(Position { x: position.x, y: position.y + 1 }) ||
    self.is_square_taken(Position { x: position.x + 1, y: position.y + 1 })
  }

  fn is_west_taken(&self, position: &Position) -> bool {
    self.is_square_taken(Position { x: position.x - 1, y: position.y - 1 }) ||
    self.is_square_taken(Position { x: position.x - 1, y: position.y }) ||
    self.is_square_taken(Position { x: position.x - 1, y: position.y + 1 })
  }

  fn is_east_taken(&self, position: &Position) -> bool {
    self.is_square_taken(Position { x: position.x + 1, y: position.y - 1 }) ||
    self.is_square_taken(Position { x: position.x + 1, y: position.y }) ||
    self.is_square_taken(Position { x: position.x + 1, y: position.y + 1 })
  }

  fn suggest_square(&self, position: &Position, directions: &[Direction; 4]) -> Option<Position> {
    let mut next_position = None;
    let mut is_free = 0;

    for direction in directions.iter().rev() {
      match direction {
        Direction::North => {
          if !self.is_north_taken(position) {
            is_free += 1;
            next_position = Some(Position { x: position.x, y: position.y - 1 });
          }
        },
        Direction::South => {
          if !self.is_south_taken(position) {
            is_free += 1;
            next_position = Some(Position { x: position.x, y: position.y + 1 });
          }
        },
        Direction::West => {
          if !self.is_west_taken(position) {
            is_free += 1;
            next_position = Some(Position { x: position.x - 1, y: position.y });
          }
        },
        Direction::East => {
          if !self.is_east_taken(position) {
            is_free += 1;
            next_position = Some(Position { x: position.x + 1, y: position.y });
          }
        },
      }
    }

    if is_free == 4 || is_free == 0 {
      None
    } else {
      next_position
    }
  }
}

enum Direction {
  North,
  South,
  West,
  East,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
  x: usize,
  y: usize,
}

fn main() {
  let input = fs::read_to_string("src/input23.txt").unwrap();
  let mut field = Field::new(input);
  let mut directions = [Direction::North, Direction::South, Direction::West, Direction::East];

  field.expand_if_needed();
  println!("{}", field);

  for _ in 0..10 {
    let mut proposals: HashMap<Position, Vec<Position>> = HashMap::new();
    
    for y in 0..field.data.len() {
      for x in 0..field.data[y].len() {
        if field.data[y][x] == '#' {
          let position = Position { x, y };
          let proposal = field.suggest_square(&position, &directions);

          if let Some(proposal) = proposal {
            proposals.entry(proposal).or_insert(Vec::new()).push(position);
          }
        }
      }
    }

    // println!("{:#?}", proposals);

    for prop in proposals.iter() {
      if prop.1.len() == 1 {
        field.data[prop.1[0].y][prop.1[0].x] = '.';
        field.data[prop.0.y][prop.0.x] = '#';
      }
    }    

    directions.rotate_left(1);
    
    // println!("{}", field);
    field.expand_if_needed();
  }

  // define minimum square containing all #
  let mut min_x = field.data[0].len();
  let mut max_x = 0;
  let mut min_y = field.data.len();
  let mut max_y = 0;

  for y in 0..field.data.len() {
    for x in 0..field.data[y].len() {
      if field.data[y][x] == '#' {
        if x < min_x {
          min_x = x;
        }
        if x > max_x {
          max_x = x;
        }
        if y < min_y {
          min_y = y;
        }
        if y > max_y {
          max_y = y;
        }
      }
    }
  }

  let mut counter: u32 = 0;
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      if field.data[y][x] == '.' {
        counter += 1;
      }
    }
  }

  println!("counter: {}", counter);
}
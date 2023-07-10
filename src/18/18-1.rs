use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vector3(i32, i32, i32);

fn main() { 
  let input = fs::read_to_string("src/input18.txt").expect("Input file exists");

  let mut grid3: Vec<Vector3> = vec![];
  let mut total = 0;

  input.lines().for_each(|line| {
    let coords = line.split(",").map(|s| s.parse::<i32>().expect("Coordinates are numbers")).collect::<Vec<_>>();
    let droplet = Vector3(coords[0], coords[1], coords[2]);

    total += 6;

    if grid3.contains(&Vector3(droplet.0 - 1, droplet.1, droplet.2)) {
      total -= 2;
    }

    if grid3.contains(&Vector3(droplet.0 + 1, droplet.1, droplet.2)) {
      total -= 2;
    }

    if grid3.contains(&Vector3(droplet.0, droplet.1 - 1, droplet.2)) {
      total -= 2;
    }

    if grid3.contains(&Vector3(droplet.0, droplet.1 + 1, droplet.2)) {
      total -= 2;
    }

    if grid3.contains(&Vector3(droplet.0, droplet.1, droplet.2 - 1)) {
      total -= 2;
    }

    if grid3.contains(&Vector3(droplet.0, droplet.1, droplet.2 + 1)) {
      total -= 2;
    }

    grid3.push(droplet);
  });

  println!("Total: {}", total);
}
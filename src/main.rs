use std::{fs, collections::HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vector3(i32, i32, i32);

fn main() { 
  let input = fs::read_to_string("src/input18.txt").expect("Input file exists");

  let mut grid3: HashSet<Vector3> = HashSet::new();
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

    grid3.insert(droplet);
  });

  println!("Total: {}", total);

  let min_vector = Vector3(
    grid3.iter().map(|v| v.0).min().unwrap() - 1,
    grid3.iter().map(|v| v.1).min().unwrap() - 1,
    grid3.iter().map(|v| v.2).min().unwrap() - 1
  );
  let max_vector: Vector3 = Vector3(
    grid3.iter().map(|v| v.0).max().unwrap() + 1,
    grid3.iter().map(|v| v.1).max().unwrap() + 1,
    grid3.iter().map(|v| v.2).max().unwrap() + 1
  );

  println!("Min: {:?}", min_vector);
  println!("Max: {:?}", max_vector);

  let mut air_set = HashSet::new();
  let mut queue = vec![min_vector];

  while queue.len() > 0 {
    let current = queue[0];
    queue.remove(0);

    air_set.insert(current);

    let mut neighbours = vec![
      Vector3(current.0 - 1, current.1, current.2),
      Vector3(current.0 + 1, current.1, current.2),
      Vector3(current.0, current.1 - 1, current.2),
      Vector3(current.0, current.1 + 1, current.2),
      Vector3(current.0, current.1, current.2 - 1),
      Vector3(current.0, current.1, current.2 + 1),
    ];

    neighbours = neighbours.into_iter().filter(|n| !grid3.contains(&n) && !air_set.contains(n) && n.0 >= min_vector.0 && n.1 >= min_vector.1 && n.2 >= min_vector.2 && n.0 <= max_vector.0 && n.1 <= max_vector.1 && n.2 <= max_vector.2).collect::<Vec<_>>();

    neighbours.into_iter().for_each(|n| {
      queue.push(n);
    });
  }

  for x in min_vector.0..=max_vector.0 {
    for y in min_vector.1..=max_vector.1 {
      for z in min_vector.2..=max_vector.2 {
        if !air_set.contains(&Vector3(x, y, z)) && !grid3.contains(&Vector3(x, y, z)) {
          println!("{:?}", Vector3(x, y, z));
        }
      }
    }
  }

}
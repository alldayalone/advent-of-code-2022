use std::{fs, collections::HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vector3(i32, i32, i32);

fn find_total_square(droplets: &HashSet<Vector3>) -> i32 {
  let mut total = 0;

  droplets.iter().for_each(|droplet| {
    total += 6;

    if droplets.contains(&Vector3(droplet.0 - 1, droplet.1, droplet.2)) {
      total -= 1;
    }

    if droplets.contains(&Vector3(droplet.0 + 1, droplet.1, droplet.2)) {
      total -= 1;
    }

    if droplets.contains(&Vector3(droplet.0, droplet.1 - 1, droplet.2)) {
      total -= 1;
    }

    if droplets.contains(&Vector3(droplet.0, droplet.1 + 1, droplet.2)) {
      total -= 1;
    }

    if droplets.contains(&Vector3(droplet.0, droplet.1, droplet.2 - 1)) {
      total -= 1;
    }

    if droplets.contains(&Vector3(droplet.0, droplet.1, droplet.2 + 1)) {
      total -= 1;
    }
  });

  total
}

fn find_exterior_square(droplets: &HashSet<Vector3>) -> i32 {
  let total = find_total_square(droplets);

  let min_vector = Vector3(
    droplets.iter().map(|v| v.0).min().unwrap(),
    droplets.iter().map(|v| v.1).min().unwrap(),
    droplets.iter().map(|v| v.2).min().unwrap()
  );
  let max_vector: Vector3 = Vector3(
    droplets.iter().map(|v| v.0).max().unwrap(),
    droplets.iter().map(|v| v.1).max().unwrap(),
    droplets.iter().map(|v| v.2).max().unwrap()
  );

  println!("Min: {:?}", min_vector);
  println!("Max: {:?}", max_vector);

  let mut air_set = HashSet::new();

  for x in min_vector.0..=max_vector.0 {
    for y in min_vector.1..=max_vector.1 {
      for z in min_vector.2..=max_vector.2 {
        if !droplets.contains(&Vector3(x, y, z)) {
          air_set.insert(Vector3(x, y, z));
        }
      }
    }
  }

  println!("Air set: {:#?}", air_set);
  println!("Air set len: {:#?}", air_set.len());

  let mut clusters: Vec<HashSet<Vector3>> = vec![HashSet::new()];

  air_set.iter().for_each(|v| {
    let mut found = false;

    if v.0 == min_vector.0 || v.0 == max_vector.0 || v.1 == min_vector.1 || v.1 == max_vector.1 || v.2 == min_vector.2 || v.2 == max_vector.2 {
      HashSet::insert(&mut clusters[0], *v);
      return;
    }

    for cluster in clusters.iter_mut() {
      if cluster.contains(&Vector3(v.0 - 1, v.1, v.2)) {
        cluster.insert(*v);
        found = true;
        break;
      }

      if cluster.contains(&Vector3(v.0 + 1, v.1, v.2)) {
        cluster.insert(*v);
        found = true;
        break;
      }

      if cluster.contains(&Vector3(v.0, v.1 - 1, v.2)) {
        cluster.insert(*v);
        found = true;
        break;
      }

      if cluster.contains(&Vector3(v.0, v.1 + 1, v.2)) {
        cluster.insert(*v);
        found = true;
        break;
      }

      if cluster.contains(&Vector3(v.0, v.1, v.2 - 1)) {
        cluster.insert(*v);
        found = true;
        break;
      }

      if cluster.contains(&Vector3(v.0, v.1, v.2 + 1)) {
        cluster.insert(*v);
        found = true;
        break;
      }
    }

    if !found {
      let mut new_cluster = HashSet::new();
      new_cluster.insert(*v);
      clusters.push(new_cluster);
    }
  });
  // println!("Clusters: {:#?}", clusters);

  println!("Clusters: {:#?}", clusters.iter().map(|c| c.len()).collect::<Vec<_>>());

  if clusters.len() == 1 {
    return total;
  }

  // iterate to merge clusters into the first one
  
  loop {
    let mut merged_clusters: Vec<HashSet<Vector3>> = vec![];
    for cluster in clusters.iter() {
      let mut found = false;

      for merged_cluster in merged_clusters.iter_mut() {
        if merged_cluster.contains(&Vector3(cluster.iter().map(|v| v.0).min().unwrap() - 1, cluster.iter().map(|v| v.1).min().unwrap(), cluster.iter().map(|v| v.2).min().unwrap())) {
          merged_cluster.extend(cluster);
          found = true;
          break;
        }

        if merged_cluster.contains(&Vector3(cluster.iter().map(|v| v.0).max().unwrap() + 1, cluster.iter().map(|v| v.1).max().unwrap(), cluster.iter().map(|v| v.2).max().unwrap())) {
          merged_cluster.extend(cluster);
          found = true;
          break;
        }

        if merged_cluster.contains(&Vector3(cluster.iter().map(|v| v.0).min().unwrap(), cluster.iter().map(|v| v.1).min().unwrap() - 1, cluster.iter().map(|v| v.2).min().unwrap())) {
          merged_cluster.extend(cluster);
          found = true;
          break;
        }

        if merged_cluster.contains(&Vector3(cluster.iter().map(|v| v.0).max().unwrap(), cluster.iter().map(|v| v.1).max().unwrap() + 1, cluster.iter().map(|v| v.2).max().unwrap())) {
          merged_cluster.extend(cluster);
          found = true;
          break;
        }

        if merged_cluster.contains(&Vector3(cluster.iter().map(|v| v.0).min().unwrap(), cluster.iter().map(|v| v.1).min().unwrap(), cluster.iter().map(|v| v.2).min().unwrap() - 1)) {
          merged_cluster.extend(cluster);
          found = true;
          break;
        }

        if merged_cluster.contains(&Vector3(cluster.iter().map(|v| v.0).max().unwrap(), cluster.iter().map(|v| v.1).max().unwrap(), cluster.iter().map(|v| v.2).max().unwrap() + 1)) {
          merged_cluster.extend(cluster);
          found = true;
          break;
        }
      }

      if !found {
        merged_clusters.push(cluster.clone());
      }
    }

    if clusters.len() == merged_clusters.len() {
      break;
    }

    clusters = merged_clusters;
  }
  

  // println!("Merged clusters: {:#?}", clusters);
  println!("Merged clusters len: {:#?}", clusters.iter().map(|c| c.len()).collect::<Vec<_>>());

  let interior_totals = clusters.iter().skip(1).map(|c| find_exterior_square(c)).collect::<Vec<_>>();

  println!("Interior totals: {:#?}", interior_totals);

  let final_answer = total - interior_totals.iter().sum::<i32>();
  println!("Final answer: {}", final_answer);

  final_answer
}

fn main() { 
  let input = fs::read_to_string("src/input18.txt").expect("Input file exists");

  let mut grid3: HashSet<Vector3> = HashSet::new();

  input.lines().for_each(|line| {
    let coords = line.split(",").map(|s| s.parse::<i32>().expect("Coordinates are numbers")).collect::<Vec<_>>();
    let droplet = Vector3(coords[0], coords[1], coords[2]);

    grid3.insert(droplet);
  });

  let exterior = find_exterior_square(&grid3);
  println!("exterior: {}", exterior);
}
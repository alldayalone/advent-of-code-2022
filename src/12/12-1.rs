use std::fs;  

fn find_point(grid: &Vec<Vec<char>>, point: char) -> (usize, usize) {
    let row = grid.iter().position(|row| row.contains(&point)).unwrap();
    let col = grid.iter().find(|row| row.contains(&point)).unwrap().iter().position(|&x| x == point).unwrap();
    (row, col)
}

fn find_neighbors(grid: &Vec<Vec<char>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let (row, col) = point;
    if row > 0  {
        neighbors.push((row - 1, col));
    }
    if row < grid.len() - 1 {
        neighbors.push((row + 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col < grid[0].len() - 1 {
        neighbors.push((row, col + 1));
    }
    neighbors.into_iter().filter(|(row, col)| grid[*row][*col] <= (grid[point.0][point.1] as u8 + 1) as char).collect()
}

fn main() { 
    let mut heightmap: Vec<Vec<char>> = fs::read_to_string("src/input12.txt").unwrap().lines().map(|line| line.chars().collect()).collect();
    // let mut dirmap: Vec<Vec<char>> = heightmap.iter().map(|row| row.iter().map(|_| '.').collect()).collect();
    let mut distmap: Vec<Vec<Option<u32>>> = heightmap.iter().map(|row| row.iter().map(|_| None).collect()).collect();

    let start_point = find_point(&heightmap, 'S');
    let end_point = find_point(&heightmap, 'E');

    heightmap[start_point.0][start_point.1] = 'a';
    heightmap[end_point.0][end_point.1] = 'z';

    // dirmap[end_point.0][end_point.1] = 'E';
    distmap[start_point.0][start_point.1] = Some(0);

    let mut queue: Vec<(usize, usize)> = vec![start_point.clone()];

    while let Some(&current_point) = queue.get(0) {
        queue.remove(0);
        let neighbors = find_neighbors(&heightmap, current_point);

        for neighbor in neighbors {
            match distmap[neighbor.0][neighbor.1] {
                Some(dist) => {
                    if dist > distmap[current_point.0][current_point.1].unwrap() + 1 {
                        distmap[neighbor.0][neighbor.1] = Some(distmap[current_point.0][current_point.1].unwrap() + 1);
                    }
                }
                None => {
                    queue.push(neighbor.clone());
                    distmap[neighbor.0][neighbor.1] = Some(distmap[current_point.0][current_point.1].unwrap() + 1);
                }
            }       
        }
    }


    println!("{:?}", distmap[end_point.0][end_point.1].unwrap());

    distmap.iter().for_each(|row| println!("{:?}", row.iter().map(|x| x.unwrap_or(0)).collect::<Vec<u32>>()));

    // convert distmap to array of u8 separated by /n
    let contents = distmap.iter().map(|row| row.iter().map(|x| x.unwrap_or(0).to_string()).collect::<Vec<_>>().join("\t")).collect::<Vec<_>>().join("\n");

    fs::write("src/output.txt", contents);
}
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input8.txt").unwrap();
    let grid = input.lines().map(|line| {
        line.chars().map(|char| char as u8 - '0' as u8).collect::<Vec<u8>>().into_boxed_slice()
    }).collect::<Vec<Box<[u8]>>>().into_boxed_slice();
    let n = grid.len();
    let m = grid[0].len();
    
    let mut highest_scenic_score = 0;
    let mut tree_spot = [0, 0];


    for i in 0..n {
        for j in 0..m{
            let top = i - (0..i).rfind(|k| grid[*k][j] >= grid[i][j]).unwrap_or(0);
            let bot = (i+1..n).find(|k| grid[*k][j] >= grid[i][j]).unwrap_or(n - 1) - i;
            let left = j - (0..j).rfind(|k| grid[i][*k] >= grid[i][j]).unwrap_or(0);
            let right = (j+1..m).find(|k| grid[i][*k] >= grid[i][j]).unwrap_or(m - 1) - j;

            let scenic_score = left * right * top * bot;

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
                tree_spot = [i, j];

                println!("New highest scenic score: {}, at ({}, {}), with top: {}, bot: {}, left: {}, right: {}", scenic_score, i, j, top, bot, left, right);
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if [i, j] == tree_spot {
                print!("\x1b[41m{}\x1b[0m", grid[i][j]);
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!()
    }

    println!("Highest scenic score: {}", highest_scenic_score);
}
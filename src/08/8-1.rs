use std::fs;

fn print_visible(digit: u8) {
    print!("\x1b[42m{}\x1b[0m", digit);
}

fn print_invisible(digit: u8) {
    print!("\x1b[41m{}\x1b[0m", digit);
}


fn main() {
    let input = fs::read_to_string("src/input8.txt").unwrap();
    let map = input.lines().map(|line| {
        line.chars().map(|char| char as u8 - '0' as u8).collect::<Vec<u8>>().into_boxed_slice()
    }).collect::<Vec<Box<[u8]>>>().into_boxed_slice();

    let mut count = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i == 0 || j == 0 || i == map.len()-1 || j == map.len()-1 {
                count += 1;
                print_visible(map[i][j]);

                continue;
            }

            if (0..i).map(|k| map[k][j]).max().unwrap_or(0) < map[i][j] {
                count += 1;
                print_visible(map[i][j]);

                continue;
            }
            if (i+1..map.len()).map(|k| map[k][j]).max().unwrap_or(0) < map[i][j] {
                count += 1;
                print_visible(map[i][j]);

                continue;
            }
            if (0..j).map(|k| map[i][k]).max().unwrap_or(0) < map[i][j] {
                count += 1;
                print_visible(map[i][j]);

                continue;
            }
            if (j+1..map.len()).map(|k| map[i][k]).max().unwrap_or(0) < map[i][j] {
                count += 1;
                print_visible(map[i][j]);
                
                continue;
            }
            print_invisible(map[i][j])
        }
        println!()
    }
    println!("Visible trees: {}", count);
}
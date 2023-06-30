use std::fs;
use std::ops::RangeInclusive;


fn range(start: usize, end: usize) -> RangeInclusive<usize> {
    start.min(end)..=start.max(end)
}

// test
// const X_START : usize = 500;
// const X_OFFSET: usize = 488;
// const X_SIZE: usize = 24;
// const Y_SIZE: usize = 12;


// tweaked :p
const X_START : usize = 500;
const X_OFFSET: usize = 320;
const X_SIZE: usize = 350;
const Y_SIZE: usize = 170;

fn display_map(map: &[[char; X_SIZE]; Y_SIZE]) {
    map.iter().for_each(|line| {
        line.iter().for_each(|c| {
            print!("{}", c);
        });
        println!();
    });
}

fn main() { 
    let mut map = [['.'; X_SIZE]; Y_SIZE];

    let mut max_y = 0;

    map[0][X_START - X_OFFSET] = '+';
    (0..X_SIZE).for_each(|i| {
        map[Y_SIZE-1][i] = '#';
    });
    let input = fs::read_to_string("src/input14.txt").unwrap();

    input.lines().for_each(|line| {
        let mut x = 0;
        let mut y = 0;

        line.split(" -> ").for_each(|pair| {
            let v: Vec<usize> = pair.split(",").map(|s| s.parse::<usize>().unwrap()).collect();

            if x == 0 && y == 0 {
                // skip first
            } else if x == v[0] {
                for i in range(y,v[1]) {
                    map[i][x - X_OFFSET] = '#';
                }
            } else if y == v[1] {
                for i in range(x, v[0]) {
                    map[y][i - X_OFFSET] = '#';
                }
            } else {
                unreachable!();
            }

            if v[1] > max_y {
                max_y = v[1];
            }

            x = v[0];
            y = v[1];
        });
    });


    let mut count = 0;

    'outer: loop {
        let mut sand_y = 0;
        let mut sand_x = X_START;

        loop {
            if map[sand_y + 1][sand_x - X_OFFSET] == '.' {
                sand_y = sand_y + 1;
            } else if sand_x - X_OFFSET >= 1 && map[sand_y + 1][sand_x - 1 - X_OFFSET] == '.' {
                sand_x = sand_x - 1;
                sand_y = sand_y + 1;
            } else if sand_x + 1 - X_OFFSET < X_SIZE && map[sand_y + 1][sand_x + 1 - X_OFFSET] == '.' {
                sand_x = sand_x + 1;
                sand_y = sand_y + 1;
            } else {
                break;
            }
        }

        map[sand_y][sand_x - X_OFFSET] = 'o';
        count += 1;

        if sand_y == 0 && sand_x == X_START {
            break 'outer;
        }
    }

    display_map(&map);
    println!("count: {}", count);
}
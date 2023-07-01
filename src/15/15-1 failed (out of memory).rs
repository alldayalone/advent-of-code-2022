use std::fs;
use regex::Regex;

// test
// const X_SIZE: usize = 28;
// const Y_SIZE: usize = 25;
// const X_OFFSET: i32 = 2;
// const Y_OFFSET: i32 = 2;
// const Y_TEST: usize = 10;

const X_SIZE: usize = 5_000_000;
const Y_SIZE: usize = 5_000_000;
const X_OFFSET: i32 = 0;
const Y_OFFSET: i32 = 0;
const Y_TEST: usize = 2_000_000;


fn _display_map(map: &[[char; X_SIZE]; Y_SIZE]) {
    map.iter().for_each(|line| {
        line.iter().for_each(|c| {
            print!("{}", c);
        });
        println!();
    });
}

fn scan(map: &mut [[char; X_SIZE]; Y_SIZE], y: usize, x: usize) -> bool {
    if x >= X_SIZE || y >= Y_SIZE {
        return false;
    }

    match map[y][x] {
        '.' => {
            map[y][x] = '#';
            false
        },
        'S' | '#' => {
            false
        },
        'B' => {
            true
        }
        _ => unreachable!()
    }
}

fn main() { 
    let mut map = [['.'; X_SIZE]; Y_SIZE];

    let input = fs::read_to_string("src/input15.txt").unwrap();
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let sensors_and_beacons = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let sensor_x = caps.get(1).map(|m| (m.as_str().parse::<i32>().unwrap() + X_OFFSET) as usize).unwrap();
        let sensor_y = caps.get(2).map(|m| (m.as_str().parse::<i32>().unwrap() + Y_OFFSET) as usize).unwrap();
        let beacon_x = caps.get(3).map(|m| (m.as_str().parse::<i32>().unwrap() + X_OFFSET) as usize).unwrap();
        let beacon_y = caps.get(4).map(|m| (m.as_str().parse::<i32>().unwrap() + Y_OFFSET) as usize).unwrap();

        ((sensor_x, sensor_y), (beacon_x, beacon_y))
    }).collect::<Vec<_>>();

    sensors_and_beacons.iter().for_each(|((sensor_x, sensor_y), (beacon_x, beacon_y))| {
        map[*sensor_y][*sensor_x] = 'S';
        map[*beacon_y][*beacon_x] = 'B';
    });

    sensors_and_beacons.iter().for_each(|((sensor_x, sensor_y), (_beacon_x, _beacon_y))| {
        let x = *sensor_x;
        let y = *sensor_y;

        for d in 1..(X_SIZE / 2) {
            let mut found = false;
            for i in 0..d {
                if y >= i && x + i >= d {
                    found = scan(&mut map, y - i, x + i - d) || found;
                }

                if x + d >= i {
                    found = scan(&mut map, y + i, x + d - i) || found;
                }

                if y + i >= d {
                    found = scan(&mut map, y + i - d, x + i) || found;
                }

                if y + d >= i && x >= i {
                    found = scan(&mut map, y + d - i, x - i) || found;
                }
            }

            if found {
                break;
            }
        }       
    });

    // display_map(&map);

    println!("Positions: {}", map[Y_TEST + Y_OFFSET as usize].iter().filter(|c| **c == '#').count());
}
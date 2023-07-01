use std::fs;
use regex::Regex;

// so, with the given constraints, that it's just a single point
// we could make a safe conclusion that the distressed beacon
// lyes on the dist + 1 from some sensor and reduce the search space
// from O(N * M) to O(S * (MAX_DIST + 1)) which is significantly smaller

// test
// const N: i32 = 20;
// const M: i32 = 20;

const N: i32 = 4_000_000;
const M: i32 = 4_000_000;

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn scan(sensors_and_beacons: &Vec<((i32, i32), (i32, i32), i32)>, y: i32, x: i32) -> bool {
    if x >= N || y >= M {
        return false;
    }
    let mut is_answer = true;

    for ((sensor_x, sensor_y), _, dist) in sensors_and_beacons {
        let dist2 = distance(*sensor_x, *sensor_y, x, y);
        if  dist2 <= *dist {
            is_answer = false;
        } 
    }

    if is_answer {
        println!("Answer: ({}, {}) = {}", x, y, x as u128 * 4_000_000 + y as u128);
    }

    is_answer
}

fn main() { 
    let input = fs::read_to_string("src/input15.txt").unwrap();
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let sensors_and_beacons = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let sensor_x = caps.get(1).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
        let sensor_y = caps.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
        let beacon_x = caps.get(3).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
        let beacon_y = caps.get(4).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();

        ((sensor_x, sensor_y), (beacon_x, beacon_y), distance(sensor_x, sensor_y, beacon_x, beacon_y))
    }).collect::<Vec<_>>();


    // println!("{:?}", sensors_and_beacons);

    for ((sensor_x, sensor_y), (_beacon_x, _beacon_y), dist) in &sensors_and_beacons {
        let x = *sensor_x;
        let y = *sensor_y;
        let d = *dist + 1;


        let mut found = false;
        for i in 0..=d {
            if y >= i && x + i >= d {
                found = scan(&sensors_and_beacons, y - i, x + i - d) || found;
            }

            if x + d >= i {
                found = scan(&sensors_and_beacons, y + i, x + d - i) || found;
            }

            if y + i >= d {
                found = scan(&sensors_and_beacons, y + i - d, x + i) || found;
            }

            if y + d >= i && x >= i {
                found = scan(&sensors_and_beacons, y + d - i, x - i) || found;
            }
        }

        if found {
            break;
        }
    }       

}
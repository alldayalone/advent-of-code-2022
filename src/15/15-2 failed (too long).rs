use std::fs;
use regex::Regex;

// test
// const N: i32 = 20;
// const M: i32 = 20;

const N: i32 = 4_000_000;
const M: i32 = 4_000_000;

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
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


    println!("{:?}", sensors_and_beacons);

    'outer: for x in 0..=N { 
        for y in 0..=M {
            let mut is_answer = true;
            for ((sensor_x, sensor_y), _, dist) in &sensors_and_beacons {
                let dist2 = distance(*sensor_x, *sensor_y, x, y);
                if  dist2 <= *dist {
                    is_answer = false;
                } 
            }

            if is_answer {
                println!("Answer: ({}, {}) = {}", x, y, x * 4_000_000 + y);
                break 'outer;

            }
        }


        println!("Progress = {}", x as f32 / N as f32);
    }
}
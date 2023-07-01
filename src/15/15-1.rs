use std::fs;
use regex::Regex;

// test
// const Y_TEST: i32 = 10;
const Y_TEST: i32 = 2_000_000;

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

    let mut counter = 0;
    // let mut test = vec![];

    let min = sensors_and_beacons.iter().min_by_key(|(_, (x,_), _)| *x).unwrap().1.0;
    let max = sensors_and_beacons.iter().max_by_key(|(_, (x,_), _)| *x).unwrap().1.0;

    println!("{:?}", sensors_and_beacons);
    println!("Min: {}, Max: {}", min, max); 
    // definitely could be optimized here but it worked
    'outer: for x in min*2..=max*2 {
        for (_, (beacon_x, beacon_y), _) in &sensors_and_beacons {
            if  (x, Y_TEST) == (*beacon_x, *beacon_y) {
                // test.push('B');
                continue 'outer;
            }
        }

        for ((sensor_x, sensor_y), _, dist) in &sensors_and_beacons {
            let dist2 = distance(*sensor_x, *sensor_y, x, Y_TEST);
            if  dist2 <= *dist {
                // println!("Sensor at ({}, {}) is closest to ({}, {}), dist = {}", sensor_x, sensor_y, x, Y_TEST, dist2);
                counter += 1;

                // test.push('#');
                continue 'outer;
            }

        }

        // test.push('.')
    }

    println!("Positions: {}", counter);
    // println!("Test: {}", test.iter().collect::<String>());
}
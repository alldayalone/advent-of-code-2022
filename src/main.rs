use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;

struct Valve {
    tag: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

#[derive(PartialEq,PartialOrd)]
struct NonNan(f64);

impl NonNan {
    fn new(val: f64) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan(val))
        }
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


fn main() { 
    
    let input = fs::read_to_string("src/input16_test.txt").unwrap();
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let valves = input.lines().map(|line| {
        let caps = re.captures(line).expect(format!("Failed to parse line {}", line).as_str());
        let tag = caps.get(1).map(|m| m.as_str().to_owned()).unwrap();
        let flow_rate = caps.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
        let tunnels = caps.get(3).map(|m| m.as_str()).unwrap().split(", ").map(|s| { s.to_owned() }).collect::<Vec<_>>();
        
        Valve { tag, flow_rate, tunnels }
    }).collect::<Vec<_>>();

    let mut distances: HashMap<(String, String), i32> = HashMap::new();

    valves.iter().for_each(|v| {
        let from_tag = v.tag.clone();
        let mut visited = vec![v.tag.clone()];
        let mut queue = vec![v.tag.clone()];

        let mut distance = 0;

        while !queue.is_empty() {
            distance += 1;
            let current_tag = queue.remove(0);

            visited.push(current_tag.clone());
            let current_valve = valves.iter().find(|v| v.tag == current_tag).unwrap();
            current_valve.tunnels.iter().for_each(|to_tag| {
                let best_distance = distances.get(&(from_tag.clone(), to_tag.clone())).unwrap_or(&i32::MAX);

                if distance < *best_distance && !from_tag.eq(to_tag) {
                    distances.insert((from_tag.clone(), to_tag.clone()), distance);
                }

                if !visited.contains(to_tag) {
                    queue.push(to_tag.clone());
                }
            });
        }
    });

    // display distances
    let mut dist = distances.iter().collect::<Vec<_>>();
    dist.sort_by_key(|((from, to), distance)| {
        (from, to)
    });
    dist.iter().for_each(|((from, to), distance)| {
        println!("{} -> {} = {}", from, to, distance);
    });

    let mut minutes: i32 = 30;
    let mut release_pressure: i32 = 0;
    let mut current_valve_tag = "AA".to_owned();
    let mut opened_valves = vec![];

    while minutes > 0 {
        let next_valve = valves.iter().filter(|v| { 
            !opened_valves.contains(&v.tag) && !v.tag.eq(&current_valve_tag)
        }).map(|v| {
            let distance = distances.get(&(current_valve_tag.clone(), v.tag.clone())).expect(format!("{} -> {}", current_valve_tag, v.tag).as_str()) + 1;
            let pressure = v.flow_rate * (minutes - distance);
            let pressure_per_distance = pressure as f64 / distance as f64;
            
            println!("{} -> {} = {}; {}; {}", current_valve_tag, v.tag, pressure, distance, pressure_per_distance);

            (v.tag.clone(), pressure, distance, pressure_per_distance)
        }).max_by_key(|(tag, flow_rate, distance, pressure_per_distance)| {
            NonNan::new(*pressure_per_distance).unwrap()
        }).unwrap();

        println!("CHOSEN {} -> {} = {}; {}; {}", current_valve_tag, next_valve.0, next_valve.1, next_valve.2, next_valve.3);

        opened_valves.push(next_valve.0.clone());
        current_valve_tag = next_valve.0.clone();
        release_pressure += next_valve.1;
        minutes -=  next_valve.2;

    }

    println!("Release pressure: {}", release_pressure);   
}
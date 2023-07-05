use std::fs;
use regex::Regex;
use serde::__private::de;
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

#[derive(Clone, Debug)]
struct SolutionTree {
    depth: u32,
    flow_rate: i32,
    pressure: i32,
    current_valve_tag: String,
    opened_valves: Vec<String>,
    closed_valves: Vec<String>,
    visited_valves: Vec<String>,
    actions: Vec<Action>,
    children: Vec<SolutionTree>
}

#[derive(Clone, Debug)]
enum Action {
    Move(String),
    Open(String)
}

fn iterate(solution_tree: &mut SolutionTree, valves: &Vec<Valve>, tracks: &HashMap<(String, String), Vec<String>>, best_solutions: &mut HashMap<(String, u32), i32>) {
    if best_solutions.get(&(solution_tree.current_valve_tag.clone(), solution_tree.depth)).unwrap_or(&0) > &solution_tree.pressure {
        return;
    } else {
        best_solutions.insert((solution_tree.current_valve_tag.clone(), solution_tree.depth), solution_tree.pressure);
    }

    let mut possible_actions = vec![];
    let current_valve = valves.iter().find(|v| v.tag == solution_tree.current_valve_tag).unwrap();

    if !solution_tree.opened_valves.contains(&current_valve.tag) && current_valve.flow_rate > 0 {
        possible_actions.push(Action::Open(current_valve.tag.clone()));
    }

    current_valve.tunnels.iter()
        .filter(|to_tag| { !solution_tree.visited_valves.contains(to_tag) })
        // .filter(|to_tag| {
        //     let closed_valves_tracks = solution_tree.closed_valves.iter().filter_map(|cv| tracks.get(&(current_valve.tag.clone(), cv.clone())));
        //     let mut next_move_candidates = closed_valves_tracks.filter_map(|track| track.first()).collect::<Vec<_>>();
            
        //     next_move_candidates.dedup();
            
        //     next_move_candidates.contains(to_tag)
        // })
        .for_each(|to_tag| {
        possible_actions.push(Action::Move(to_tag.clone()));
    });

    let mut children = possible_actions.iter().filter_map(|action| {
        let mut solution_branch = solution_tree.clone();

        solution_branch.depth += 1;
        solution_branch.pressure += solution_branch.flow_rate;
        solution_branch.actions.push(action.clone());

        match action {
            Action::Move(to_tag) => {
                solution_branch.current_valve_tag = to_tag.clone();
            },
            Action::Open(to_tag) => {
                solution_branch.opened_valves.push(to_tag.clone());
                solution_branch.closed_valves.retain(|v| v != to_tag);
                solution_branch.flow_rate += current_valve.flow_rate;
                solution_branch.visited_valves = vec![];
            }
        }

        solution_branch.visited_valves.push(current_valve.tag.clone());

        // if valves.iter().all(|v| solution_branch.opened_valves.contains(&v.tag)) {
        //     solution_branch.pressure += solution_branch.flow_rate;
        // }

        Some(solution_branch)
    }).collect::<Vec<_>>();
    
    children.into_iter().for_each(|solution_branch| {
        solution_tree.children.push(solution_branch);
    });
}

fn recursive(solution_tree: &mut SolutionTree, valves: &Vec<Valve>, tracks: &HashMap<(String, String), Vec<String>>, best_solutions: &mut HashMap<(String, u32), i32>) {
    if solution_tree.depth >= 30 {
        return;
    }

    iterate(solution_tree, valves, tracks, best_solutions);

    solution_tree.children.iter_mut().for_each(|child| {
        recursive(child, valves, tracks, best_solutions);
    });
}

fn print_leafs(solution_tree: &SolutionTree) {
    if solution_tree.children.is_empty() && solution_tree.pressure > 0 {
        println!("{:#?}", solution_tree);
    } else {
        solution_tree.children.iter().for_each(|child| {
            print_leafs(child);
        });
    }
}

fn find_best_leaf(solution_tree: &SolutionTree, best: &mut Option<SolutionTree>) {
    if solution_tree.children.is_empty() {
        match best {
            None => {
                *best = Some(solution_tree.clone());
            },
            Some(current_best) => {
                if solution_tree.pressure > current_best.pressure {
                    *best = Some(solution_tree.clone());
                }
            }
        }
    } else {
        solution_tree.children.iter().for_each(|child| {
            find_best_leaf(child, best);
        });
    }
}
// it's time to make optimization when you go only to open vents with optimal path

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
    let mut tracks: HashMap<(String, String), Vec<String>> = HashMap::new();

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
                    let mut track = tracks.get(&(from_tag.clone(), current_valve.tag.clone())).unwrap_or(&vec![]).clone();

                    track.push(to_tag.clone());
                    distances.insert((from_tag.clone(), to_tag.clone()), distance);
                    tracks.insert((from_tag.clone(), to_tag.clone()), track);
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
        println!("{} -> {} = {}; {:?}", from, to, distance, tracks.get(&(from.clone(), to.clone())));
    });


    let mut solution_tree = SolutionTree {
        depth: 0,
        flow_rate: 0,
        pressure: 0,
        current_valve_tag: "AA".to_owned(),
        opened_valves: vec![],
        closed_valves: valves.iter().filter(|v| v.flow_rate > 0).map(|v| v.tag.clone()).collect::<Vec<_>>(),
        visited_valves: vec![],
        actions: vec![],
        children: vec![]
    };

    let mut best_solutions: HashMap<(String, u32), i32> = HashMap::new();

    recursive(&mut solution_tree, &valves, &tracks, &mut best_solutions);

    // print_leafs(&solution_tree);
    let mut best_leaf = None;

    find_best_leaf(&solution_tree, &mut best_leaf);

    println!("{:#?}", best_leaf);
    // println!("{:#?}", solution_tree);





    // let valves.iter().filter(|v| v.flow_rate > 0)

    // let mut released_pressure: i32 = 0;
    // let mut pressure_per_minute = 0;
    // let mut current_valve_tag = "AA".to_owned();
    // let mut opened_valves = vec![];

    // for minutes_left in (1..=30).rev() {
    //     let next_valve = valves.iter().filter(|v| { 
    //         !opened_valves.contains(&v.tag) && !v.tag.eq(&current_valve_tag)
    //     }).map(|v| {
    //         let distance = distances.get(&(current_valve_tag.clone(), v.tag.clone())).expect(format!("{} -> {}", current_valve_tag, v.tag).as_str()) + 1;
    //         let pressure = v.flow_rate * (minutes_left - distance);
    //         let pressure_per_distance = pressure as f64 / distance as f64;
            
    //         println!("{} -> {} = {}; {}; {}", current_valve_tag, v.tag, pressure, distance, pressure_per_distance);

    //         (v.tag.clone(), pressure, distance, pressure_per_distance)
    //     }).max_by_key(|(tag, flow_rate, distance, pressure_per_distance)| {
    //         NonNan::new(*pressure_per_distance).unwrap()
    //     }).unwrap();

    //     println!("CHOSEN {} -> {} = {}; {}; {}", current_valve_tag, next_valve.0, next_valve.1, next_valve.2, next_valve.3);

    //     opened_valves.push(next_valve.0.clone());
    //     current_valve_tag = next_valve.0.clone();
    //     released_pressure += pressure_per_minute;
    //     // minutes -=  next_valve.2;
    // }

    // 

    // println!("Minutes: {}", minutes);
    // println!("Release pressure: {}", release_pressure);   
}
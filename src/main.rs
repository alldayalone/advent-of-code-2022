use std::fs;
use regex::Regex;
use std::collections::HashMap;

const DEPTH: u32 = 26;

struct Valve {
    tag: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

#[derive(Clone, Debug)]
struct SolutionTree {
    depth: u32,
    flow_rate: i32,
    pressure: i32,
    current_valve_tag: (String, String),
    aim_valve_tag: (Option<String>, Option<String>),
    opened_valves: Vec<String>,
    closed_valves: Vec<String>,
    actions: (Vec<Action>, Vec<Action>)
}

#[derive(Clone, Debug)]
enum Action {
    Open(String),
    Move(String),
}

static mut BEST_PRESSURE: i32 = 0;
fn solve<Count: FnMut()>(solution_tree: &SolutionTree, valves: &Vec<Valve>, tracks: &HashMap<(String, String), Vec<String>>, count: &mut Count) {
    if solution_tree.depth >= DEPTH {
        if solution_tree.pressure > unsafe { BEST_PRESSURE } {
            unsafe { BEST_PRESSURE = solution_tree.pressure; }
            println!("Best tree: {:#?}", solution_tree);
            println!("Best pressure: {}", unsafe { BEST_PRESSURE });
        }
        return;
    }
    count();

    if solution_tree.closed_valves.is_empty() {
        let dist_cutted = DEPTH - solution_tree.depth;
        let mut solution_branch = solution_tree.clone();

        solution_branch.depth += dist_cutted;
        solution_branch.pressure += (dist_cutted as i32) * solution_branch.flow_rate;

        for _ in 0..dist_cutted {
            solution_branch.actions.0.push(Action::Move("Random".to_owned()));
            solution_branch.actions.1.push(Action::Move("Random".to_owned()));
        }


        solve(&solution_branch, valves, tracks, count);
        return
    }

    match &solution_tree.aim_valve_tag {
        (Some(aim_tag0), Some(aim_tag1)) => {
            let track0 = tracks.get(&(solution_tree.current_valve_tag.0.to_owned(), aim_tag0.to_owned())).unwrap();
            let track1 = tracks.get(&(solution_tree.current_valve_tag.1.to_owned(), aim_tag1.to_owned())).unwrap();

            let dist = track0.len().min(track1.len())  as u32 + 1;
    
            if solution_tree.depth + dist >= DEPTH { 
                let dist_cutted = DEPTH - solution_tree.depth;
                let mut solution_branch = solution_tree.clone();
    
                solution_branch.depth += dist_cutted;
                solution_branch.pressure += (dist_cutted as i32) * solution_branch.flow_rate;
                solution_branch.current_valve_tag = (
                    track0.get(dist_cutted as usize - 1).unwrap_or(track0.last().unwrap()).to_owned(),
                    track1.get(dist_cutted as usize - 1).unwrap_or(track1.last().unwrap()).to_owned(),
                );

                for _ in 0..dist_cutted {
                    solution_branch.actions.0.push(Action::Move("Random".to_owned()));
                    solution_branch.actions.1.push(Action::Move("Random".to_owned()));
                }
    
                solve(&solution_branch, valves, tracks, count);
            } else {
                let mut solution_branch = solution_tree.clone();

                solution_branch.depth += dist;
                solution_branch.pressure += (dist as i32) * solution_branch.flow_rate;

                for i in 0..track0.len().min(dist as usize) {
                    solution_branch.actions.0.push(Action::Move(track0.get(i).unwrap().to_owned()));
                }

                for i in 0..track1.len().min(dist as usize) {
                    solution_branch.actions.1.push(Action::Move(track1.get(i).unwrap().to_owned()));
                }

                if dist > track0.len() as u32 {
                    let last_valve_tag = track0.last().unwrap_or(&solution_branch.current_valve_tag.0);
                    let last_valve  = valves.iter().find(|v| &v.tag == last_valve_tag).unwrap();

                    solution_branch.flow_rate += last_valve.flow_rate;
                    solution_branch.opened_valves.push(last_valve.tag.to_owned());
                    solution_branch.closed_valves.retain(|v| v != &last_valve.tag);
                    solution_branch.aim_valve_tag.0 = None;
                    solution_branch.current_valve_tag.0 = last_valve_tag.to_owned();

                    solution_branch.actions.0.push(Action::Open(solution_branch.current_valve_tag.0.to_owned()));
                } else {
                    solution_branch.current_valve_tag.0 = track0.get(dist as usize - 1).unwrap().to_owned();

                }

                if dist > track1.len() as u32 {
                    let last_valve_tag = track1.last().unwrap_or(&solution_branch.current_valve_tag.1);
                    let last_valve  = valves.iter().find(|v| &v.tag == last_valve_tag).unwrap();

                    solution_branch.flow_rate += last_valve.flow_rate;
                    solution_branch.opened_valves.push(last_valve.tag.to_owned());
                    solution_branch.closed_valves.retain(|v| v != &last_valve.tag);
                    solution_branch.aim_valve_tag.1 = None;
                    solution_branch.current_valve_tag.1 = last_valve_tag.to_owned();
                    solution_branch.actions.1.push(Action::Open(solution_branch.current_valve_tag.1.to_owned()));

                } else {
                    solution_branch.current_valve_tag.1 = track1.get(dist as usize - 1).unwrap().to_owned();
                }
    
                solve(&solution_branch, valves, tracks, count);
            }
        },
        (Some(aim_tag), None) => {
            solution_tree.closed_valves.iter()
                .filter(|new_aim_tag| new_aim_tag != &aim_tag && new_aim_tag != &&solution_tree.current_valve_tag.1)
                .for_each(|new_aim_tag| {
                    let mut solution_branch = solution_tree.clone();

                    solution_branch.aim_valve_tag = (Some(aim_tag.to_owned()), Some(new_aim_tag.to_owned()));
                    
                    solve(&solution_branch, valves, tracks, count);
                });
        },
        (None, Some(aim_tag)) => {
            solution_tree.closed_valves.iter()
                .filter(|new_aim_tag| new_aim_tag != &aim_tag && new_aim_tag != &&solution_tree.current_valve_tag.0)
                .for_each(|new_aim_tag| {
                    let mut solution_branch = solution_tree.clone();

                    solution_branch.aim_valve_tag = (Some(new_aim_tag.to_owned()), Some(aim_tag.to_owned()));
                    
                    solve(&solution_branch, valves, tracks, count);
                });
        },
        (None, None) => {
            solution_tree.closed_valves.iter()
                .filter(|new_aim_tag| new_aim_tag != &&solution_tree.current_valve_tag.0)
                .for_each(|aim_tag| {
                    let mut solution_branch = solution_tree.clone();

                    solution_branch.aim_valve_tag = (Some(aim_tag.to_owned()), None);
                    
                    solve(&solution_branch, valves, tracks, count);
                });
        },
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

    valves.iter().filter(|v| v.flow_rate > 0).map(|v| v.tag.clone()).for_each(|tag| {
        distances.insert((tag.clone(), tag.clone()), 0);
        tracks.insert((tag.clone(), tag.clone()), vec![]);
    });

    // display distances
    let mut dist = distances.iter().collect::<Vec<_>>();
    dist.sort_by_key(|((from, to), _)| {
        (from, to)
    });
    dist.iter().for_each(|((from, to), distance)| {
        println!("{} -> {} = {}; {:?}", from, to, distance, tracks.get(&(from.clone(), to.clone())));
    });


    let solution_tree = SolutionTree {
        depth: 0,
        flow_rate: 0,
        pressure: 0,
        current_valve_tag: ("AA".to_owned(), "AA".to_owned()),
        aim_valve_tag: (None, None),
        opened_valves: vec![],
        closed_valves: valves.iter().filter(|v| v.flow_rate > 0).map(|v| v.tag.clone()).collect::<Vec<_>>(),
        actions: (vec![], vec![])
    };


    let mut count = 0;
    solve(&solution_tree, &valves, &tracks, &mut || count += 1);

    println!("Total calls: {}", count);
    println!("Best pressure: {}", unsafe { BEST_PRESSURE });
}
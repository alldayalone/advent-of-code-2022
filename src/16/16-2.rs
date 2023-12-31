use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::time::SystemTime;
// use std::backtrace::Backtrace;

const DEPTH: u32 = 26;
const IDLE_VALVE: u8 = 53;

struct Valve<'valve> {
    tag: &'valve str,
    flow_rate: i32,
    tunnels: Vec<&'valve str>,
}

#[derive(Clone, Debug)]
struct SolutionTree {
    depth: u32,
    flow_rate: i32,
    pressure: i32,
    current_valve_tag: (u8, u8),
    aim_valve_tag: (Option<u8>, Option<u8>),
    opened_closed_valves: u64 // closed 1, opened 0
    // actions: (Vec<Action>, Vec<Action>)
}

// #[derive(Clone, Debug)]
// enum Action {
//     Open(String),
//     Move(String),
//     Idle
// }

static mut estimated_best_solution: i32 = 0;
static mut max_flow_rate: i32 = 0;

fn is_worse(solution_tree: &SolutionTree) -> bool {
  unsafe { (solution_tree.pressure + max_flow_rate * (DEPTH - solution_tree.depth) as i32) < estimated_best_solution }
}

fn is_better(solution_tree: &SolutionTree) {
  let estimate = solution_tree.pressure + solution_tree.flow_rate * (DEPTH - solution_tree.depth) as i32;

  unsafe {
    if estimate > estimated_best_solution {
      estimated_best_solution = estimate;  
    }
  }
}

static mut BEST_PRESSURE: i32 = 0;
fn solve<Count: FnMut()>(solution_tree: &SolutionTree, valves: &Vec<Valve>, tracks: &HashMap<u16, Vec<u8>>, count: &mut Count) {
    if is_worse(solution_tree) {
      return;
    }

    is_better(solution_tree);


    if solution_tree.depth >= DEPTH {
        if solution_tree.pressure > unsafe { BEST_PRESSURE } {
            unsafe { BEST_PRESSURE = solution_tree.pressure; }
            // println!("Best tree: {:#?}", solution_tree);
            println!("Best pressure: {}", unsafe { BEST_PRESSURE });
        }
        return;
    }
    count();

    // if solution_tree.closed_valves.is_empty() {
    //     let dist_cutted = DEPTH - solution_tree.depth;
    //     let mut solution_branch = solution_tree.clone();

    //     solution_branch.depth = DEPTH;
    //     solution_branch.pressure += (dist_cutted as i32) * solution_branch.flow_rate;

    //     solution_branch.actions.0.push(Action::Idle);
    //     solution_branch.actions.1.push(Action::Idle);

    //     solve(&solution_branch, valves, tracks, count);
    //     return
    // }

    match &solution_tree.aim_valve_tag {
        (Some(aim_tag0), Some(aim_tag1)) => {
            let track0 = tracks.get(&concat_tags(solution_tree.current_valve_tag.0, *aim_tag0));
            let track1 = tracks.get(&concat_tags(solution_tree.current_valve_tag.1, *aim_tag1));

            let track0len = track0.map(|v| v.len() as u32).unwrap_or(u32::MAX - 1);
            let track1len = track1.map(|v| v.len() as u32).unwrap_or(u32::MAX - 1);

            let dist = track0len.min(track1len) + 1;
    
            if dist >= DEPTH - solution_tree.depth { 
                let dist_cutted = DEPTH - solution_tree.depth;
                let mut solution_branch = solution_tree.clone();
    
                solution_branch.depth = DEPTH;
                solution_branch.pressure += (dist_cutted as i32) * solution_branch.flow_rate;

                // solution_branch.actions.0.push(Action::Idle);
                // solution_branch.actions.1.push(Action::Idle);
    
                solve(&solution_branch, valves, tracks, count);
            } else {
                let mut solution_branch = solution_tree.clone();

                solution_branch.depth += dist;
                solution_branch.pressure += (dist as i32) * solution_branch.flow_rate;

                // for i in 0..track0len.min(dist) {
                //     solution_branch.actions.0.push(track0.map(|t| Action::Move(t.get(i as u8).unwrap().to_owned())).unwrap_or(Action::Idle));
                // }

                // for i in 0..track1len.min(dist) {
                //     solution_branch.actions.1.push(track1.map(|t| Action::Move(t.get(i as u8).unwrap().to_owned())).unwrap_or(Action::Idle));
                // }

                if dist > track0len as u32 {
                    let last_valve_tag = track0.unwrap().last().unwrap_or(&solution_branch.current_valve_tag.0);
                    let last_valve  = valves.get(*last_valve_tag as usize).unwrap();

                    solution_branch.flow_rate += last_valve.flow_rate;
                    solution_branch.opened_closed_valves &= !(1 << last_valve_tag); // 0 corresponding bit
                    solution_branch.aim_valve_tag.0 = None;
                    solution_branch.current_valve_tag.0 = last_valve_tag.to_owned();

                    // solution_branch.actions.0.push(Action::Open(solution_branch.current_valve_tag.0.to_owned()));
                } else {
                    solution_branch.current_valve_tag.0 = track0.map(|t| t.get(dist as usize - 1).unwrap().to_owned()).unwrap_or(solution_branch.current_valve_tag.0);

                }

                if dist > track1len as u32 {
                    let last_valve_tag = track1.unwrap().last().unwrap_or(&solution_branch.current_valve_tag.1);
                    let last_valve  = valves.get(*last_valve_tag as usize).unwrap();

                    solution_branch.flow_rate += last_valve.flow_rate;
                    solution_branch.opened_closed_valves &= !(1 << last_valve_tag); // 0 corresponding bit
                    solution_branch.aim_valve_tag.1 = None;
                    solution_branch.current_valve_tag.1 = last_valve_tag.to_owned();
                    // solution_branch.actions.1.push(Action::Open(solution_branch.current_valve_tag.1.to_owned()));

                } else {
                    solution_branch.current_valve_tag.1 = track1.map(|t| t.get(dist as usize - 1).unwrap().to_owned()).unwrap_or(solution_branch.current_valve_tag.1);
                }
    
                solve(&solution_branch, valves, tracks, count);
            }
        },
        (Some(aim_tag), None) => {
            if u64::count_ones(solution_tree.opened_closed_valves) <= 1 {
                let mut solution_branch = solution_tree.clone();

                solution_branch.aim_valve_tag = (Some(aim_tag.to_owned()), Some(IDLE_VALVE));
                
                solve(&solution_branch, valves, tracks, count);
            } else {
                let masked = solution_tree.opened_closed_valves & !(1 << aim_tag) & !(1 << solution_tree.current_valve_tag.1);
                
                for new_aim_tag in 0..64 as u8 {
                    if masked & (1 << new_aim_tag) > 0 {
                        let mut solution_branch = solution_tree.clone();

                        solution_branch.aim_valve_tag = (Some(*aim_tag), Some(new_aim_tag));
                        
                        solve(&solution_branch, valves, tracks, count);
                    }
                }
            }
        },
        (None, Some(aim_tag)) => {
            if u64::count_ones(solution_tree.opened_closed_valves) <= 1 {
                let mut solution_branch = solution_tree.clone();

                solution_branch.aim_valve_tag = (Some(IDLE_VALVE), Some(aim_tag.to_owned()));
                
                solve(&solution_branch, valves, tracks, count);
            } else {
                let masked = solution_tree.opened_closed_valves & !(1 << aim_tag) & !(1 << solution_tree.current_valve_tag.0);

                for new_aim_tag in 0..64 as u8 {
                    if masked & (1 << new_aim_tag) > 0 {
                        let mut solution_branch = solution_tree.clone();

                        solution_branch.aim_valve_tag = (Some(new_aim_tag), Some(*aim_tag));
                        
                        solve(&solution_branch, valves, tracks, count);
                    }
                }
            }
        },
        (None, None) => {
            if solution_tree.opened_closed_valves == 0  {
                let mut solution_branch = solution_tree.clone();

                solution_branch.aim_valve_tag = (Some(IDLE_VALVE), Some(IDLE_VALVE));
                
                solve(&solution_branch, valves, tracks, count);
            } else {
                let masked = solution_tree.opened_closed_valves & !(1 << solution_tree.current_valve_tag.0);

                for new_aim_tag in 0..64 as u8 {
                    if masked & (1 << new_aim_tag) > 0 {
                        let mut solution_branch = solution_tree.clone();

                        solution_branch.aim_valve_tag = (Some(new_aim_tag), None);
                        
                        solve(&solution_branch, valves, tracks, count);
                    }
                }
            }
        },
    }
}

fn tag_to_position(valves: &Vec<Valve>, tag: &str) -> u8 {
    valves.iter().position(|v| v.tag == tag).expect("Valve with that tag not found") as u8
}

fn concat_tags(from: u8, to: u8) -> u16 {
    ((from as u16) << 8) + to as u16
}

fn main() { 
    
    let input = fs::read_to_string("src/16/input16.txt").unwrap();
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let valves = input.lines().map(|line| {
        let caps = re.captures(line).expect(format!("Failed to parse line {}", line).as_str());
        let tag = caps.get(1).map(|m| m.as_str()).unwrap();
        let flow_rate = caps.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
        let tunnels = caps.get(3).map(|m| m.as_str()).unwrap().split(", ").collect::<Vec<_>>();
        
        Valve { tag, flow_rate, tunnels }
    }).collect::<Vec<_>>();

    let mut distances: HashMap<u16, i32> = HashMap::new();
    let mut tracks: HashMap<u16, Vec<u8>> = HashMap::new();

    valves.iter().enumerate().for_each(|(index, _)| {
        let from_tag = index as u8;
        let mut visited = vec![from_tag];
        let mut queue = vec![from_tag];

        let mut distance = 0;

        while !queue.is_empty() {
            distance += 1;
            let current_tag = queue.remove(0);

            visited.push(current_tag);
            let current_valve = valves.get(current_tag as usize).unwrap();
            current_valve.tunnels.iter().for_each(|to_tag_raw| {
                let to_tag = valves.iter().position(|v| v.tag == *to_tag_raw).unwrap() as u8;
                let best_distance = distances.get(&concat_tags(from_tag, to_tag)).unwrap_or(&i32::MAX);

                if distance < *best_distance && from_tag != to_tag {
                    let mut track = tracks.get(&concat_tags(from_tag, current_tag)).unwrap_or(&vec![]).clone();

                    track.push(to_tag.clone());
                    distances.insert(concat_tags(from_tag, to_tag), distance);
                    tracks.insert(concat_tags(from_tag, to_tag), track);
                }

                if !visited.contains(&to_tag) {
                    queue.push(to_tag);
                }
            });
        }
    });

    valves.iter().enumerate().filter(|(_, v)| v.flow_rate > 0).for_each(|(index, _)| {
        distances.insert(concat_tags(index as u8, index as u8), 0);
        tracks.insert(concat_tags(index as u8, index as u8), vec![]);
    });

    // display distances
    let mut dist = distances.iter().collect::<Vec<_>>();
    dist.sort_by_key(|(key, _)| {
        **key
    });
    dist.iter().for_each(|(key, distance)| {
        println!("{} -> {} = {}; {:?}", *key >> 8, (*key << 8) >> 8, distance, tracks.get(key));
    });

    unsafe { max_flow_rate = valves.iter().map(|v| v.flow_rate).sum(); }


    let solution_tree = SolutionTree {
        depth: 0,
        flow_rate: 0,
        pressure: 0,
        current_valve_tag: (tag_to_position(&valves, "AA"), tag_to_position(&valves, "AA")),
        aim_valve_tag: (None, None),
        opened_closed_valves: valves.iter().enumerate().filter(|(_, v)| v.flow_rate > 0).map(|(index, _)| index as u8).fold(0, |acc, v| acc | (1 << v)),
        // actions: (vec![], vec![])
    };


    let start = SystemTime::now();
    let mut count = 0;
    let mut count_fn = || {
        count += 1;
        if count % 1_000_000 == 0 {
            println!("{}, calls total: {}M", SystemTime::now().duration_since(start).unwrap().as_secs(), count / 1_000_000);
        }
    };
    solve(&solution_tree, &valves, &tracks, &mut count_fn);

    println!("Total calls: {}", count);
    println!("Best pressure: {}", unsafe { BEST_PRESSURE });
}
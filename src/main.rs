use std::collections::VecDeque;
use std::vec;
use std::{fs, ops::Add};
use regex::Regex;

use queues::*;


const MINUTES: u32 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Resource(u32, u32, u32, u32);

impl Resource {
  fn is_enough(self, other: Self) -> bool {
    self.0 >= other.0 && self.1 >= other.1 && self.2 >= other.2 && self.3 >= other.3
  }

  fn add(self, other: Self) -> Self {
    Resource(
      self.0 + other.0,
      self.1 + other.1,
      self.2 + other.2,
      self.3 + other.3,
    )
  }

  fn diff(self, other: Self) -> Self {
    Resource(
      self.0 - other.0,
      self.1 - other.1,
      self.2 - other.2,
      self.3 - other.3,
    )
  }

  fn eq(self, other: Self) -> bool {
    self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
  }

  fn le(self, other: Self) -> bool {
    self.0 <= other.0 && self.1 <= other.1 && self.2 <= other.2 && self.3 <= other.3
  }

  fn l(self, other: Self) -> bool {
    (self.0 < other.0 || self.1 < other.1 || self.2 < other.2 || self.3 < other.3) && self.le(other)
  }

  fn ge(self, other: Self) -> bool {
    self.0 >= other.0 && self.1 >= other.1 && self.2 >= other.2 && self.3 >= other.3
  }

  fn diff_safe(self, other: Self) -> Self {
    Resource(
      if self.0 > other.0 { self.0 - other.0 } else { 0 },
      if self.1 > other.1 { self.1 - other.1 } else { 0 },
      if self.2 > other.2 { self.2 - other.2 } else { 0 },
      if self.3 > other.3 { self.3 - other.3 } else { 0 },
    )
  }

  fn scalar_multiply(self, scalar: u32) -> Self {
    Resource(
      self.0 * scalar,
      self.1 * scalar,
      self.2 * scalar,
      self.3 * scalar,
    )
  }

  fn scalar_div(self, scalar: u32) -> Self {
    Resource(
      self.0 / scalar,
      self.1 / scalar,
      self.2 / scalar,
      self.3 / scalar,
    )
  }

  // self = production
  fn time_to_build(self, costs: Self) -> Option<u32> {
    let mut times = vec![];

    if costs.0 > 0 {
      if self.0 == 0 { return None }
      times.push((costs.0 + self.0 - 1 )/ self.0);
    }

    if costs.1 > 0 {
      if self.0 == 0 { return None }
      times.push((costs.1  + self.1 - 1 )/ self.1);
    }

    if costs.2 > 0 {
      if self.0 == 0 { return None }
      times.push((costs.2 + self.2 - 1) / self.2);
    }

    if costs.3 > 0 {
      if self.0 == 0 { return None }
      times.push((costs.3 + self.3 - 1) / self.3);
    }

    Some(times.into_iter().max().unwrap_or(0))
  }
}

impl Add for Resource {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Resource(
          self.0 + other.0,
          self.1 + other.1,
          self.2 + other.2,
          self.3 + other.3,
        )
    }
}

fn is_worse(state: &State, &other: &State) -> bool {
  if state.minute == 1 {
    return false;
  }
  let future = state.production.scalar_multiply(MINUTES + 1 - state.minute ).add(state.resources);
  let other_future = other.production.scalar_multiply(MINUTES + 1 - other.minute).add(other.resources);

  if future.3 == other_future.3 {
    return state.production.l(other.production)
  } else {
    return future.3 < other_future.3;
  }
}

fn is_better(state: &State, &other: &State) -> bool {
  let future = state.production.scalar_multiply(MINUTES + 1 - state.minute ).add(state.resources);
  let other_future = other.production.scalar_multiply(MINUTES + 1 - other.minute).add(other.resources);

  if future.3 == other_future.3 {
    return state.production.ge(other.production);
  } else {
    return future.3 > other_future.3;
  }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint(Resource, Resource, Resource, Resource);

#[derive(Debug, Clone, Copy)]
struct State {
  minute: u32,
  resources: Resource,
  production: Resource,
}

fn create_resource(kind: &str, quantity: u32) -> Resource {
  match kind {
    "ore" => Resource(quantity, 0, 0, 0),
    "clay" => Resource(0, quantity, 0, 0),
    "obsidian" => Resource(0, 0, quantity, 0),
    "geode" => Resource(0, 0, 0, quantity),
    _ => panic!("Unknown resource")
  }
}

fn parse_input() -> Vec<Blueprint> {
  let input = fs::read_to_string("src/input19.txt").expect("File exists");
  let mut blueprints: Vec<Blueprint> = vec![];

  let robot_re = Regex::new(r"(\w+) robot costs (\d+) (\w+)( and (\d+) (\w+))?\.").unwrap();

  input.lines().for_each(|line| {
    let mut constr = line.split(" Each ");

    constr.next();
    let mut robots: Vec<Resource> = vec![];

    loop {
      let robot_line = constr.next().unwrap_or_default();

      match robot_re.captures(robot_line) {
        Some(caps) => {
          let mut costs = create_resource(caps.get(3).unwrap().as_str(), caps.get(2).unwrap().as_str().parse::<u32>().unwrap());

          if caps.get(4).is_some() {
            costs = costs + create_resource(caps.get(6).unwrap().as_str(), caps.get(5).unwrap().as_str().parse::<u32>().unwrap());
          }
          
          robots.push(costs);       
        },
        None => break
      }
    }

    blueprints.push(Blueprint(robots[0], robots[1], robots[2], robots[3]));
  });
  
  blueprints
}


fn iterate(blueprint: &Blueprint, state: &State, best_state_by_minute: &mut [State; MINUTES as usize + 2], backtrack: &str) {

  if is_worse(&state, &best_state_by_minute[state.minute as usize]) {
    // println!("skip");
    return;
  }

  if is_better(&state, &best_state_by_minute[state.minute as usize]) {
    println!("State: {:?}", state);
    println!("Backtrack: {:?}", backtrack);

    best_state_by_minute[state.minute as usize] = state.clone();
  }

  // println!("State: {:?}", state);
  if state.minute > MINUTES {
    // println!("Finish. Resources: {:?}", state.resources);
    return;
  }

  if state.production.2 > 0 {
    // Geode case
    let geode_robot_costs = blueprint.3;
    match state.production.time_to_build(geode_robot_costs.diff_safe(state.resources)) {
      Some(minutes_to_produce) => {
        if state.minute + minutes_to_produce < MINUTES {
          iterate(blueprint, &State {
            minute: state.minute + minutes_to_produce + 1, 
            resources: state.resources.add(state.production.scalar_multiply(minutes_to_produce + 1)).diff(geode_robot_costs),
            production: state.production + Resource(0,0,0,1)
          }, best_state_by_minute, &[backtrack, "G"].join(""));
        // } else {
        //   let minutes_left = MINUTES - state.minute;
        //   iterate(blueprint, &State {
        //     minute: state.minute + minutes_left + 1, 
        //     resources: state.resources.add(state.production.scalar_multiply(minutes_left + 1)),
        //     production: state.production.clone()
        //   }, best_state_by_minute, &[backtrack, "I"].join(""));
        //   return;
        }
      },
      None => {
        println!("Not enough resources to produce geode robots");
      }
    }
  }

  if state.production.1 > 0 {
    // Obsidian case
    let obsidian_robot_costs = blueprint.2;
    match state.production.time_to_build(obsidian_robot_costs.diff_safe(state.resources)) {
      Some(minutes_to_produce) => {
        if state.minute + minutes_to_produce < MINUTES {
          iterate(blueprint, &State {
            minute: state.minute + minutes_to_produce + 1, 
            resources: state.resources.add(state.production.scalar_multiply(minutes_to_produce + 1)).diff(obsidian_robot_costs),
            production: state.production + Resource(0,0,1,0)
          }, best_state_by_minute, &[backtrack, "B"].join(""));
        }
      },
      None => {
        println!("Not enough resources to produce obsidian robots");
      }
    }
  }

  // Clay case
  let clay_robot_costs = blueprint.1;
  match state.production.time_to_build(clay_robot_costs.diff_safe(state.resources)) {
    Some(minutes_to_produce) => {
      if state.minute + minutes_to_produce < MINUTES {
        iterate(blueprint, &State {
          minute: state.minute + minutes_to_produce + 1, 
          resources: state.resources.add(state.production.scalar_multiply(minutes_to_produce + 1)).diff(clay_robot_costs),
          production: state.production + Resource(0,1,0,0)
        }, best_state_by_minute, &[backtrack, "C"].join(""));
      }
    },
    None => {
      println!("Not enough resources to produce clay robots");
    }
  }

  // Ore case
  let ore_robot_costs = blueprint.0;
  match state.production.time_to_build(ore_robot_costs.diff_safe(state.resources)) {
    Some(minutes_to_produce) => {
      if state.minute + minutes_to_produce < MINUTES {
        iterate(blueprint, &State {
          minute: state.minute + minutes_to_produce + 1, 
          resources: state.resources.add(state.production.scalar_multiply(minutes_to_produce + 1)).diff(ore_robot_costs),
          production: state.production + Resource(1,0,0,0)
        }, best_state_by_minute, &[backtrack, "O"].join(""));
      }
    },
    None => {
      println!("Not enough resources to produce ore robots");
    }
  }

  // Idle case
  iterate(blueprint, &State {
    minute: state.minute + 1,
    resources: state.resources.add(state.production),
    production: state.production.clone()
  }, best_state_by_minute, &[backtrack, "I"].join(""));
}

fn main() {

  // unit test resource methods
  // add
  assert_eq!(Resource(1,2,3,4).add(Resource(1,2,3,4)), Resource(2,4,6,8));
  assert_eq!(Resource(1,2,3,4).add(Resource(0,0,0,0)), Resource(1,2,3,4));
  assert_eq!(Resource(1,2,3,4).add(Resource(0,0,0,1)), Resource(1,2,3,5));
  assert_eq!(Resource(1,2,3,4).add(Resource(0,0,1,0)), Resource(1,2,4,4));
  assert_eq!(Resource(1,2,3,4).add(Resource(0,1,0,0)), Resource(1,3,3,4));
  assert_eq!(Resource(1,2,3,4).add(Resource(1,0,0,0)), Resource(2,2,3,4));
  assert_eq!(Resource(1,2,3,4).add(Resource(1,1,1,1)), Resource(2,3,4,5));
  // diff
  assert_eq!(Resource(1,2,3,4).diff(Resource(1,2,3,4)), Resource(0,0,0,0));
  assert_eq!(Resource(1,2,3,4).diff(Resource(0,0,0,0)), Resource(1,2,3,4));
  assert_eq!(Resource(1,2,3,4).diff(Resource(0,0,0,1)), Resource(1,2,3,3));
  assert_eq!(Resource(1,2,3,4).diff(Resource(0,0,1,0)), Resource(1,2,2,4));
  assert_eq!(Resource(1,2,3,4).diff(Resource(0,1,0,0)), Resource(1,1,3,4));
  assert_eq!(Resource(1,2,3,4).diff(Resource(1,0,0,0)), Resource(0,2,3,4));
  assert_eq!(Resource(1,2,3,4).diff(Resource(1,1,1,1)), Resource(0,1,2,3));
  // scalar_multiply
  assert_eq!(Resource(1,2,3,4).scalar_multiply(0), Resource(0,0,0,0));
  assert_eq!(Resource(1,2,3,4).scalar_multiply(1), Resource(1,2,3,4));
  assert_eq!(Resource(1,2,3,4).scalar_multiply(2), Resource(2,4,6,8));
  assert_eq!(Resource(1,2,3,4).scalar_multiply(3), Resource(3,6,9,12));
  assert_eq!(Resource(1,2,3,4).scalar_multiply(4), Resource(4,8,12,16));
  // time_to_build
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(1,2,3,4)), Some(1));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(0,0,0,0)), Some(0));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(0,0,0,1)), Some(1));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(0,0,1,0)), Some(1));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(0,1,0,0)), Some(1));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(1,0,0,0)), Some(1));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(1,1,1,1)), Some(1));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(2,2,2,2)), Some(2));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(3,3,3,3)), Some(3));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(4,4,4,4)), Some(4));
  assert_eq!(Resource(1,2,3,4).time_to_build(Resource(5,5,5,5)), Some(5));
  assert_eq!(Resource(2,2,3,4).time_to_build(Resource(6,6,6,6)), Some(3));
  assert_eq!(Resource(2,2,3,4).time_to_build(Resource(7,7,7,7)), Some(4));
  assert_eq!(Resource(2,2,3,4).time_to_build(Resource(8,8,8,8)), Some(4));

  let blueprints = parse_input();

  println!("Blueprints: {:?}", blueprints);  
  let initial_state = State {
    minute: 1, 
    resources: Resource(0,0,0,0), 
    production: Resource(1,0,0,0)
  };


  let answer: u32 = blueprints.iter().enumerate().map(|(i, blueprint)| {
    let mut best_state_by_minute = [initial_state; MINUTES as usize + 2];
    iterate(blueprint, &initial_state, &mut best_state_by_minute, "");

    println!("Blueprint {}: {:?}", i, best_state_by_minute.last());

    best_state_by_minute.last().unwrap().resources.3
  }).enumerate().map(|(i, geodes)| {
    (i as u32 + 1) * geodes
  }).sum();

  println!("Answer: {}", answer);
}
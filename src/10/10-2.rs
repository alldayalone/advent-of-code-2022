use std::fs;  

const N: usize = 6;
const M: usize = 40;

fn left_pad(s: String, n: usize) -> String {
    let mut result = String::new();
    for _ in 0..(n-s.len()) {
        result.push(' ');
    }
    result.push_str(s.to_string().as_str());
    result
}

fn print_sprite_position(register_x: i32) {
    let prefix = (1..register_x).map(|_| '.').collect::<String>();
    let postfix = (register_x..38).map(|_| '.').collect::<String>();
    println!("Sprite  position: {}###{}", prefix, postfix);
}

fn print_start_cycle(cycle: usize, running_command: &str) {
    println!("Start cycle  {}: begin executing {}", left_pad(cycle.to_string(), 3), running_command);
}

fn print_during_cycle(cycle: usize, crt: [[char; M]; N], i: usize) {
    println!("During cycle {}: CRT draws pixel in position {}", left_pad(cycle.to_string(), 3), cycle - 1);
    println!("Current CRT  row: {}", crt[i].iter().collect::<String>());
}

fn print_end_of_cycle(cycle: usize, running_command: &str, register_x: i32) {
    println!("End of cycle {}: finish executing {} (Register X is now {})", left_pad(cycle.to_string(), 3), running_command, register_x);
}

fn main() { 
    let input = fs::read_to_string("src/input10.txt").unwrap();
    let mut command_iter = input.lines();
    
    let mut register_x: i32 = 1;
    let mut running_command: Option<&str> = None;
    let mut cycles_to_skip = 0;
    // let mut output = 0;
   
    let mut crt: [[char; M]; N] = [[' '; M]; N];
    for row in crt.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    print_sprite_position(register_x);

    for cycle in 1..=240 {
        println!();

        // Take new command if not running
        match running_command {
            Some(_) => {}
            None => {
                match command_iter.next() {
                    Some(cmd_line) => {
                        print_start_cycle(cycle, cmd_line);
                        let mut cmd_args = cmd_line.split_whitespace();
        
                        match cmd_args.next() {
                            Some("noop") => {
                                cycles_to_skip = 0;
                                running_command = Some(cmd_line);
                            },
                            Some("addx") => {
                                cycles_to_skip = 1;
                                running_command = Some(cmd_line);
                            },
                            Some(_) => { panic!("Unknown command") },
                            _ => { panic!("Empty cmd line") }
                        }
                    },
                    None => { break }
                }
            }
        }

        // Observe
        // if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        //     let signal_strength = (cycle as i32) * register_x;
        //     println!("Cycle {}th, signal strength={}", cycle, signal_strength);
        //     output += signal_strength;
        // }

        // Draw CRT
        let i = (cycle - 1) / M;
        let j = (cycle - 1) % M;
        crt[i][j] = if (j as i32) < register_x - 1 || (j  as i32) > register_x + 1 { '.' } else { '#' };
        print_during_cycle(cycle, crt, i);

        // Command might take some cycles to run
        if cycles_to_skip > 0 {
            cycles_to_skip -= 1;
            continue
        }

        // Run command
        match running_command {
            Some(cmd_line) => {
                let mut cmd_args = cmd_line.split_whitespace();

                match cmd_args.next() {
                    Some("noop") => {},
                    Some("addx") => {
                        register_x += cmd_args.next().unwrap().parse::<i32>().unwrap();

                        print_end_of_cycle(cycle, cmd_line, register_x);
                        print_sprite_position(register_x);
                    },
                    Some(_) => { panic!("Unknown command") },
                    _ => { panic!("Empty cmd line") }
                }

                running_command = None;
            }
            None => {}
        }
    }

    // println!("Sum: {}", output);
    println!();

    for row in crt.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}
use std::fs;  

fn main() { 
    let input = fs::read_to_string("src/input10.txt").unwrap();
    let mut command_iter = input.lines();
    
    let mut register_x: i32 = 1;
    let mut running_command: Option<&str> = None;
    let mut cycles_to_skip = 0;
    let mut output = 0;

    for cycle in 1.. {
        // Take new command if not running
        match running_command {
            Some(_) => {}
            None => {
                match command_iter.next() {
                    Some(cmd_line) => {
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
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            let signal_strength = cycle * register_x;
            println!("Cycle {}th, signal strength={}", cycle, signal_strength);
            output += signal_strength;
        }

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
                    },
                    Some(_) => { panic!("Unknown command") },
                    _ => { panic!("Empty cmd line") }
                }

                running_command = None;
            }
            None => {}
        }
    }

    println!("Sum: {}", output);
}
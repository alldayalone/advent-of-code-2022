use std::fs;  

#[derive(Clone)]
struct Monkey {
    name: String,
    items: Vec<u64>,
    increase_worry_level: fn(&u64) -> u64,
    inspect_item: fn(&u64) -> usize,
}


fn main() { 
    // let input = fs::read_to_string("src/input10.txt").unwrap();
    let mut monkeys: Vec<Monkey> = Vec::new();
    
    // input.lines().collect::<Vec<_>>().split(|line| line == &"\n").for_each(|lines| {
    //     let mut iter = lines.iter();

    //     let name = iter.next().unwrap().to_string().replace(":", "").to_lowercase();
    //     let items = iter.next().unwrap().split_at(18).1.split(", ").map(|item| {
    //         item.parse::<u64>().unwrap()
    //     }).collect::<Vec<_>>();

    //     let operation = match iter.next().unwrap() {
    //         "Starting"
    //     };
    //     let test = || { }; // iter.next();
            
    //     Monkey::new(name, items, operation, test)
    // }).collect::<Vec<_>>();


    // monkeys.push(Monkey {
    //     name: "Monkey 0".to_string(),
    //     items: vec![79, 98],
    //     increase_worry_level: |worry_level| {
    //         return worry_level * 19
    //     },
    //     inspect_item: |item| {
    //         if item % 23 == 0 { 2 } else { 3 }
    //     },
    // });

    // monkeys.push(Monkey {
    //     name: "Monkey 1".to_string(),
    //     items: vec![54, 65, 75, 74],
    //     increase_worry_level: |worry_level| {
    //         return worry_level + 6
    //     },
    //     inspect_item: |item| {
    //         if item % 19 == 0 { 2 } else { 0 }
    //     },
    // });

    // monkeys.push(Monkey {
    //     name: "Monkey 2".to_string(),
    //     items: vec![79, 60, 97],
    //     increase_worry_level: |worry_level| {
    //         worry_level * worry_level
    //     },
    //     inspect_item: |item| {
    //         if item % 13 == 0 { 1 } else { 3 }
    //     },
    // });

    // monkeys.push(Monkey {
    //     name: "Monkey 3".to_string(),
    //     items: vec![74],
    //     increase_worry_level: |worry_level| {
    //         worry_level + 3
    //     },
    //     inspect_item: |item| {
    //         if item % 17 == 0 { 0 } else { 1 }
    //     },
    // });

        // prod data
    monkeys.push(Monkey {
        name: "Monkey 0".to_string(),
        items: vec![66, 79],
        increase_worry_level: |worry_level| {
            worry_level * 11
        },
        inspect_item: |item| {
            if item % 7 == 0 { 6 } else { 7 }
        },
    });

    monkeys.push(Monkey {
        name: "Monkey 1".to_string(),
        items: vec![84, 94, 94, 81, 98, 75],
        increase_worry_level: |worry_level| {
            worry_level * 17
        },
        inspect_item: |item| {
            if item % 13 == 0 { 5 } else { 2 }
        },
    });


    monkeys.push(Monkey {
        name: "Monkey 2".to_string(),
        items: vec![85, 79, 59, 64, 79, 95, 67],
        increase_worry_level: |worry_level| {
            worry_level + 8
        },
        inspect_item: |item| {
            if item % 5 == 0 { 4 } else { 5 }
        },
    });

    monkeys.push(Monkey {
        name: "Monkey 3".to_string(),
        items: vec![70],
        increase_worry_level: |worry_level| {
            worry_level + 3
        },
        inspect_item: |item| {
            if item % 19 == 0 { 6 } else { 0 }
        },
    });

    monkeys.push(Monkey {
        name: "Monkey 4".to_string(),
        items: vec![57, 69, 78, 78],
        increase_worry_level: |worry_level| {
            worry_level + 4
        },
        inspect_item: |item| {
            if item % 2 == 0 { 0 } else { 3 }
        },
    });

    monkeys.push(Monkey {
        name: "Monkey 5".to_string(),
        items: vec![65, 92, 60, 74, 72],
        increase_worry_level: |worry_level| {
            worry_level + 7
        },
        inspect_item: |item| {
            if item % 11 == 0 { 3 } else { 4 }
        },
    });

    monkeys.push(Monkey {
        name: "Monkey 6".to_string(),
        items: vec![77, 91, 91],
        increase_worry_level: |worry_level| {
            worry_level * worry_level
        },
        inspect_item: |item| {
            if item % 17 == 0 { 1 } else { 7 }
        },
    });

    monkeys.push(Monkey {
        name: "Monkey 7".to_string(),
        items: vec![76, 58, 57, 55, 67, 77, 54, 99],
        increase_worry_level: |worry_level| {
            worry_level + 6
        },
        inspect_item: |item| {
            if item % 3 == 0 { 2 } else { 1 }
        },
    });

    let mut inspection_counts = [0;8];

    for round in 1..=10000 {
        (0..monkeys.len()).for_each(|monkey_index| {
            let monkey = monkeys[monkey_index].clone();
            // println!("{}:", monkey.name);

            for item in monkey.items {
                inspection_counts[monkey_index] += 1;
                // println!(" Monkey inspects an item with a worry level of {}.", item);

                let mut new_item = (monkey.increase_worry_level)(&item);
                // println!("  Worry level is X to {}.", new_item);

                new_item = new_item % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19);
                // println!("  Monkey gets bored with item. Worry level is divided by 3 to {}.", new_item);
                // println!("  Current worry level.");

                let new_owner = (monkey.inspect_item)(&new_item);
                monkeys[new_owner].items.push(new_item);
                // println!("  Item with worry level {} is thrown to monkey {}.", new_item, new_owner);
            }

            monkeys[monkey_index].items.clear();
        });

        if (round % 1000 == 0) {
            println!("== After round {} ==", round);
            inspection_counts.iter().enumerate().for_each(|(index, count)| {
                println!("Monkey {} inspected items {} items.", index, count);
            });


            // println!("\nAfter round {}, the monkeys are holding items with these worry levels:", round);
            // monkeys.iter().for_each(|monkey| {
            //     println!("{}: {:?}", monkey.name, monkey.items);
            // });
            println!();
        }
    }

    inspection_counts.sort();


    let monkey_business_level = inspection_counts.iter().rev().take(2).product::<u64>();

    println!("\nMonkey business level over 20 rounds: {}", monkey_business_level);

}
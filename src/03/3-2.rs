use std::fs;
use std::collections::HashSet;

fn get_char_priority(c: &char) -> u32 {
    let char_code = *c as u32; // is it ok?
    if char_code > 96 { char_code - 96 } else { char_code - 38 }
}


fn main() {
    let input = fs::read_to_string("src/input3.txt").unwrap();
    let elf_groups = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;

    elf_groups.chunks(3).for_each(|elf_group|{
        let badges = elf_group.iter()
            .map(|line| { line.chars().collect::<HashSet<char>>() })
            .reduce(|acc, other| {
                 acc.intersection(&other).copied().collect()
            });
           
        for dup in badges.unwrap().iter() {
            let priority = get_char_priority(dup);

            sum += priority;
        }
        
    });

   println!("Total sum: {}", sum);
}

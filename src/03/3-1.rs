use std::fs;
use std::collections::HashSet;

fn get_char_priority(c: &char) -> u32 {
    let char_code = *c as u32; // is it ok?
    if char_code > 96 { char_code - 96 } else { char_code - 38 }
}


fn main() {
    let input = fs::read_to_string("src/input3.txt").unwrap();
    let mut sum = 0;

    input.lines().for_each(|line| {
        let (comp_a, comp_b) = line.split_at(line.len() / 2);
        let set_a: HashSet<char>  = comp_a.chars().collect();
        let set_b: HashSet<char>  = comp_b.chars().collect();

        for dup in set_a.intersection(&set_b) {
            let priority = get_char_priority(dup);

            sum += priority;
        }      
    });


   println!("Total sum: {}", sum);
}

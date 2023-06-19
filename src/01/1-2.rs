use std::fs;

fn main() {
    let input = fs::read_to_string("src/input2.txt").unwrap();
    // declare a vector of i32
    let mut elves: Vec<i32> = Vec::new();
    let mut sum = 0;
    input.lines().for_each(|line| {
        if line == "" {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    });

    elves.sort_unstable_by(|a, b| b.cmp(a));

    elves.truncate(3);
    
    let top_three_sum: i32 = elves.iter().sum();

    println!("Top three elves sum: {}", top_three_sum);

    // println!("Elf carrying most calories: {}. Number of calories: {}", elf_max, sum_max)
}

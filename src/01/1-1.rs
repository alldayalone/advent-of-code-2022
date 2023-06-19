use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut elf_max = 1;
    let mut elf_count = 1;
    let mut sum = 0;
    let mut sum_max = 0;
    input.lines().for_each(|line| {
        if line == "" {
            if sum > sum_max {
                elf_max = elf_count;
                sum_max = sum;
            }

            elf_count += 1;
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    });

    println!("Elf carrying most calories: {}. Number of calories: {}", elf_max, sum_max)
}

use std::fs;

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn popn(&mut self, n: usize) -> Option<Vec<T>> {
        let mut result = Vec::new();
        for _ in 0..n {
            match self.pop() {
                Some(item) => result.push(item),
                None => break,
            }
        }
        Some(result)
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}



fn main() {
    let input = fs::read_to_string("src/input5.txt").unwrap();

    let crates = input.lines().take_while(|line| !line.is_empty()).collect::<Vec<_>>();
    let moves = input.lines().skip(crates.len() + 1).collect::<Vec<_>>();

    let mut stacks: Vec<Stack<char>> = (0..(crates[0].len() / 4 + 1)).map(|_| Stack { data: Vec::new() }).collect();

    crates.iter().rev().skip(1).for_each(|line| {
        line.chars().skip(1).step_by(4).enumerate().for_each(|(index, ch)| {
            if ch != ' ' {
                stacks[index].push(ch)
            }
        })
    });

    moves.iter().for_each(|line| {
        let mut words = line.split_whitespace();

        let n = words.nth(1).unwrap().parse::<usize>().unwrap();
        let from = words.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = words.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        let popped = stacks[from].popn(n).unwrap();
        
        popped.iter().for_each(|ch| stacks[to].push(*ch));
    });

    // print top items of stacks
    stacks.iter().for_each(|stack| {
        match stack.peek() {
            Some(ch) => print!("{}", ch),
            None => unreachable!()
        }
    });

    println!();
}

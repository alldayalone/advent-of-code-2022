use std::fs;
use std::cmp::Ordering;

#[derive(Debug,PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

fn get_score_by_shape(shape: &Shape)->i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_score_by_result(result: &Ordering)->i32 {
    match result {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6
    }
}

impl From<&str> for Shape {
    fn from(str: &str) -> Self {
        match str {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => unreachable!()
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       return Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
       match (self, other) {
        (Shape::Rock, Shape::Rock) => Ordering::Equal,
        (Shape::Rock, Shape::Paper) => Ordering::Less,
        (Shape::Rock, Shape::Scissors) => Ordering::Greater,
        (Shape::Scissors, Shape::Rock) => Ordering::Less,
        (Shape::Scissors, Shape::Paper) => Ordering::Greater,
        (Shape::Scissors, Shape::Scissors) => Ordering::Equal,
        (Shape::Paper, Shape::Rock) => Ordering::Greater,
        (Shape::Paper, Shape::Paper) => Ordering::Equal,
        (Shape::Paper, Shape::Scissors) => Ordering::Less,
       }
    }
}

fn main() {
    let input = fs::read_to_string("src/input4.txt").unwrap();
    
    let mut total_score = 0;

    input.lines().for_each(|line| {
        let mut iter = line.split_ascii_whitespace();

        let opponent_shape: Shape = iter.next().unwrap().into();
        let my_shape: Shape = iter.next().unwrap().into();
        let result = my_shape.cmp(&opponent_shape);

        
        let score_by_shape = get_score_by_shape(&my_shape);
        let score_by_result  = get_score_by_result(&result);
        let score = score_by_shape + score_by_result;
        
        // println!("Opponent: {:?}\t Me: {:?}\t Result: {:?}\t Score by shape: {}\t Score by result: {}\t Total score: {}", opponent_shape, my_shape, result, score_by_shape, score_by_result, score);
        total_score += score;
    });


   println!("Total score: {}", total_score);
}

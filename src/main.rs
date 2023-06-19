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
            _ => unreachable!()
        }
    }
}

fn parse_result(result: &str)->Ordering {
    match result {
        "X" => Ordering::Less,
        "Y" => Ordering::Equal,
        "Z" => Ordering::Greater,
        _ => unreachable!()
    }
}

fn get_shape_for_result(shape: &Shape, other: &Ordering) -> Shape {
    match (shape, other) {
    (Shape::Rock, Ordering::Equal) => Shape::Rock, 
    (Shape::Rock, Ordering::Less) => Shape::Scissors, 
    (Shape::Rock, Ordering::Greater) => Shape::Paper, 
    (Shape::Scissors, Ordering::Less) => Shape::Paper, 
    (Shape::Scissors, Ordering::Greater) => Shape::Rock, 
    (Shape::Scissors, Ordering::Equal) => Shape::Scissors, 
    (Shape::Paper, Ordering::Greater) => Shape::Scissors, 
    (Shape::Paper, Ordering::Equal) => Shape::Paper, 
    (Shape::Paper, Ordering::Less) => Shape::Rock, 
    }
}


fn main() {
    let input = fs::read_to_string("src/input4.txt").unwrap();
    
    let mut total_score = 0;

    input.lines().for_each(|line| {
        let mut iter = line.split_ascii_whitespace();

        let opponent_shape: Shape = iter.next().unwrap().into();
        let result: Ordering = parse_result(iter.next().unwrap());
        let my_shape: Shape = get_shape_for_result(&opponent_shape, &result);

        
        let score_by_shape = get_score_by_shape(&my_shape);
        let score_by_result  = get_score_by_result(&result);
        let score = score_by_shape + score_by_result;
        
        // println!("Opponent: {:?}\t Me: {:?}\t Result: {:?}\t Score by shape: {}\t Score by result: {}\t Total score: {}", opponent_shape, my_shape, result, score_by_shape, score_by_result, score);
        total_score += score;
    });


   println!("Total score: {}", total_score);
}

use serde::{Deserialize, Serialize};
use std::fs;  
use std::fmt;
use std::cmp::Ordering;

#[derive(Serialize, Deserialize,Debug,Clone)]
#[serde(untagged)]
enum Signal {
    Integer(i32),
    List(Vec<Signal>)
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signal::Integer(val) => write!(f, "{}", val),
            Signal::List(vec) => {
                write!(f, "[").unwrap();
                vec.iter().take(1).for_each(|sig| { write!(f, "{}", sig).unwrap() });
                vec.iter().skip(1).for_each(|sig| { write!(f, ",{}", sig).unwrap() });
                write!(f, "]").unwrap();
                Ok(())
            }
        }
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Signal::Integer(a), Signal::Integer(b)) => a.eq(b),
            (Signal::List(a), Signal::List(b)) => a.eq(b),
            (Signal::Integer(_), Signal::List(_)) => other.eq(&Signal::List(vec![self.clone()])),
            (Signal::List(_), Signal::Integer(_)) => self.eq(&Signal::List(vec![other.clone()])),

        }
    }
}
impl Eq for Signal {}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Signal::Integer(a), Signal::Integer(b)) => a.partial_cmp(b),
            (Signal::List(a), Signal::List(b)) => {
                for (c,b) in a.iter().zip(b.iter()) {
                    match c.partial_cmp(b) {
                        Some(Ordering::Equal) => continue,
                        Some(Ordering::Less) => return Some(Ordering::Less),
                        Some(Ordering::Greater) => return Some(Ordering::Greater),
                        None => return None
                    }
                }

                a.len().partial_cmp(&b.len())
            },
            (Signal::Integer(_), Signal::List(_)) => Signal::List(vec![self.clone()]).partial_cmp(other),
            (Signal::List(_), Signal::Integer(_)) => self.partial_cmp(&Signal::List(vec![other.clone()])),

        }
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Signal::Integer(a), Signal::Integer(b)) => a.cmp(b),
            (Signal::List(a), Signal::List(b)) => {
                for (c,b) in a.iter().zip(b.iter()) {
                    match c.cmp(b) {
                        Ordering::Equal => continue,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                       
                    }
                }

                a.len().cmp(&b.len())
            },
            (Signal::Integer(_), Signal::List(_)) => Signal::List(vec![self.clone()]).cmp(other),
            (Signal::List(_), Signal::Integer(_)) => self.cmp(&Signal::List(vec![other.clone()])),

        }
    }
}

fn main() { 
    let input = fs::read_to_string("src/input13.txt").unwrap();

    let mut lines = input.lines();

    let mut packages: Vec<Signal> = vec![];   

    let first_divider = Signal::List(vec![Signal::List(vec![Signal::Integer(2)])]);
    let second_divider = Signal::List(vec![Signal::List(vec![Signal::Integer(6)])]);

    packages.push(first_divider.clone());
    packages.push(second_divider.clone());

    while let Some(first) = lines.next() {
        let second = lines.next().unwrap();

        let first_parsed: Signal = serde_json::from_str(first).unwrap();
        let second_parsed: Signal = serde_json::from_str(second).unwrap();

        packages.push(first_parsed);
        packages.push(second_parsed);

        lines.next();
    }

    packages.sort();

    let first_divider = packages.iter().position(|p| { *p == first_divider }).unwrap() + 1;
    let second_divider = packages.iter().position(|p| { *p == second_divider }).unwrap() + 1;

    // for p in packages {
    //     println!("{}", p)
    // }

    println!("First divider: {}", first_divider);
    println!("Second divider: {}", second_divider);
    println!("Decoder key: {}", first_divider * second_divider);
}
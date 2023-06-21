use std::fs;

fn main() {
    let input = fs::read_to_string("src/input6_test.txt").unwrap();
    let mut it = input.chars().enumerate();
    
    let mut window: Vec<char> = vec![];
    
    while let Some((index, char)) = it.next() {
        if Some(&char) == window.get(0) {
            window.remove(0);
        } else if Some(&char) == window.get(1) {
            window.remove(0);
            window.remove(0);
        } else if Some(&char) == window.get(2) {
            window.remove(0);
            window.remove(0);
            window.remove(0);
        } else if window.len() == 4 {
            println!("Message starts at {}", index);
            break;
        }

        window.push(char);
    }
}
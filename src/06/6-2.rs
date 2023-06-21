use std::fs;

fn main() {
    let input = fs::read_to_string("src/input6.txt").unwrap();

    input.lines().for_each(|line| {
        let mut chars = line.chars().enumerate();
        let mut window: Vec<char> = vec![];
        
        while let Some((index, char)) = chars.next() {
            let duplicate = window.iter().enumerate().rfind(|(_, wchar)| wchar == &&char);
    
            if let Some((dup_index, _)) = duplicate {
                window.splice(0..dup_index, []);
            }
            
            if window.len() == 14 {
                println!("Message starts at {}", index + 1);
                break;
            }
    
            window.push(char);
        }
    })
}
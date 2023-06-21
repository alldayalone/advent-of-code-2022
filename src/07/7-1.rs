use std::fs;

struct File {
    path: String,
    size: u128,
}

fn main() {
    let input = fs::read_to_string("src/input7.txt").unwrap();
    let mut files: Vec<File> = Vec::new();
    let mut dirs: Vec<String> = vec![String::from("/")];
    let mut current_dir = String::from("");

    input.lines().for_each(|line| {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        match words.get(0) {
            Some(&"$") => {
                match words.get(1) {
                    Some(&"cd") => {
                        match words.get(2) {
                            Some(&"/") => {
                                current_dir = String::from("");
                            }
                            Some(&"..") => {
                                let mut path = current_dir.split("/").collect::<Vec<&str>>();
                                path.pop();
                                current_dir = path.join("/");
                            }
                            Some(&str) => {
                                current_dir = format!("{}/{}", &current_dir, str);
                            }
                            None => unreachable!()
                        }
                    }
                    Some(&"ls") => {}
                    Some(_) => unreachable!(),
                    None => unreachable!()
                }
            }
            Some(&"dir") => {
                let dir = format!("{}/{}", &current_dir, words.get(1).unwrap());
                dirs.push(dir);
            }
            Some(str) => {
                let file = File {
                    path: format!("{}/{}", &current_dir, words.get(1).unwrap()),
                    size: str.parse::<u128>().unwrap(),
                };
                files.push(file);
            }
            None => unreachable!()
        }
    });

    files.sort_by(|a, b| a.path.cmp(&b.path));

    let dirs_with_sizes = dirs.iter().filter_map(|dir| {
        let total_size = files.iter().filter_map(|file| {
            if file.path.starts_with(&format!("{}/", &dir)) {
                Some(file.size)
            } else {
                None
            }
        }).sum::<u128>();

        if total_size < 100_000 {
            Some((dir, total_size))
        } else {
            None
        }
    }).collect::<Vec<(&String, u128)>>();

    let sum_of_totals = dirs_with_sizes.iter().map(|(_, total)| total).sum::<u128>();

    println!("Sum of totals {}", sum_of_totals)
}
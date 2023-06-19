use std::fs;

struct Range {
    from: i32,
    to: i32
}

impl From<&str> for Range {
    fn from(str: &str) -> Self{
        let range: Vec<i32> = str.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        return Range { from: range[0], to: range[1] };
    }
}


fn main() {
    let mut count = 0;
    let input = fs::read_to_string("src/input4.txt").unwrap();

    input.lines().for_each(|line| {
        let mut iter = line.split(|c| c == ',' || c == '-');

        let range_a = Range {
            from: iter.next().unwrap().parse::<i32>().unwrap(), 
            to: iter.next().unwrap().parse::<i32>().unwrap()
        };
        let range_b = Range { 
            from: iter.next().unwrap().parse::<i32>().unwrap(), 
            to: iter.next().unwrap().parse::<i32>().unwrap() 
        };

        if (range_a.from <= range_b.from && range_a.to >= range_b.to) || 
            (range_a.from >= range_b.from && range_a.to <= range_b.to) {
                count += 1;
        }
    });

    println!("Total: {}", count);
}

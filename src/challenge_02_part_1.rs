use std::fs::read_to_string;

pub fn main() {
    let file = read_to_string("input.txt").unwrap();
    let ranges = file.split(",").collect::<Vec<&str>>();
    let mut invalid_ids = Vec::<u64>::new();

    for range in ranges {
        let parts = range.split("-").collect::<Vec<&str>>();
        let first_integer = parts[0].parse::<u64>().unwrap();
        let second_integer = parts[1].trim().parse::<u64>().unwrap();
        for i in first_integer..=second_integer {
            let string = i.to_string();
            if string.len() % 2 == 0 {
                let first_half = &string[0..string.len()/2];
                let second_half = &string[string.len()/2..];
                if first_half == second_half {
                    invalid_ids.push(i);
                }
            }
        }
    }

    let result: u64 = invalid_ids.iter().sum();
    println!("{}", result);
}
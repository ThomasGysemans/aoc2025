use std::collections::HashSet;
use std::fs::read_to_string;

pub fn main() {
    let file = read_to_string("input.txt").unwrap();
    let mut ranges: Vec<[u64; 2]> = Vec::new();
    let mut fresh_ingredients: HashSet<u64> = HashSet::new();
    let mut line_index_of_ingredients = 0;

    for (index, line) in file.lines().enumerate() {
        if line.trim().is_empty() {
            line_index_of_ingredients = index + 1;
            break;
        } else {
            let split_string = line.split("-").collect::<Vec<&str>>();
            let start = split_string[0].parse::<u64>().unwrap();
            let end = split_string[1].parse::<u64>().unwrap();
            ranges.push([start, end]);
        }
    }

    for line in file.lines().skip(line_index_of_ingredients) {
        let string = line.trim().to_string();
        let number = string.parse::<u64>().unwrap();
        if !fresh_ingredients.contains(&number) {
            for range in ranges.iter() {
                let start = range[0];
                let end = range[1];
                if number >= start && number <= end {
                    fresh_ingredients.insert(number);
                }
            }
        }
    }

    println!("{}", fresh_ingredients.len());
}
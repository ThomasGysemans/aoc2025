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
            if i <= 10 { continue; }
            if is_invalid_id(i) {
                invalid_ids.push(i);
            }
        }
    }

    let result: u64 = invalid_ids.iter().sum();
    println!("{}", result);
}

fn is_invalid_id(number: u64) -> bool {
    let string = number.to_string();
    let divisibles = get_useful_divisibles(string.len() as u64);
    for divisible in divisibles {
        let parts = split_string(&string, divisible as usize);
        let mut is_invalid = true;
        for j in 1..parts.len() {
            if parts[0] != parts[j] {
                is_invalid = false;
            }
        }
        if is_invalid {
            println!("pushing {}", number);
            return true;
        }
    }
    false
}

fn split_string(string: &String, n: usize) -> Vec<String> {
    string.chars()
        .collect::<Vec<_>>()
        .chunks(n)
        .map(|c| c.iter().collect())
        .collect()
}

fn get_useful_divisibles(number: u64) -> Vec<u64> {
    if number <= 3 { return vec![1]; }
    let mut divisibles: Vec<u64> = vec![1];
    let sqrt = (number as f64).sqrt().floor() as u64;
    for i in 2..=sqrt {
        if number % i == 0 {
            divisibles.push(i);
            divisibles.push(number / i);
        }
    }
    divisibles
}
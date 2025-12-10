use std::fs::read_to_string;

pub fn main() {
    let file = read_to_string("input.txt").unwrap();
    let mut joltages = Vec::<u32>::new();

    for line in file.lines() {
        let string = line.trim().to_string();
        let mut max = string.chars().next().unwrap().to_string().parse::<u32>().unwrap();
        let mut max_index = 0;
        for i in 1..string.len()-1 {
            let number = string.chars().nth(i).unwrap().to_string().parse::<u32>().unwrap();
            if number > max {
                max = number;
                max_index = i;
            }
        }
        let first_digit = max;
        max = string.chars().nth(max_index+1).unwrap().to_string().parse::<u32>().unwrap();
        for j in max_index+1..string.len() {
            let number = string.chars().nth(j).unwrap().to_string().parse::<u32>().unwrap();
            if number > max {
                max = number;
            }
        }
        let second_digit = max;
        let joltage = format!("{}{}", first_digit, second_digit).parse::<u32>().unwrap();
        println!("{}", joltage);
        joltages.push(joltage);
    }

    let result: u32 = joltages.iter().sum();
    println!("{}", result);
}
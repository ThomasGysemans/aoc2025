use std::fs::read_to_string;

pub fn main() {
    let file = read_to_string("input.txt").unwrap();
    let mut joltages = Vec::<u64>::new();

    for line in file.lines() {
        let bank = line.trim().to_string();
        let mut string = "".to_string();
        let mut last_char_index = 0;
        for i in 0..12 {
            let bank_slice = &bank[last_char_index..=bank.len() - (12 - i)];
            let mut max = bank_slice.chars().next().unwrap().to_string().parse::<u64>().unwrap();
            let mut max_index = 0;
            for j in 1..bank_slice.len() {
                let char = bank_slice.chars().nth(j).unwrap();
                let digit = char.to_string().parse::<u64>().unwrap();
                if digit > max {
                    max = digit;
                    max_index = j;
                }
            }
            last_char_index += max_index + 1;
            string = format!("{}{}", string, max);
        }
        joltages.push(string.parse::<u64>().unwrap());
    }

    let result: u64 = joltages.iter().sum();
    println!("{}", result);
}
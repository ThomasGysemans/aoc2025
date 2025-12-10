use std::fs::read_to_string;

pub fn main() {
    let file = read_to_string("input.txt").unwrap();
    let mut dial = 50i32;
    let mut result = 0u32;

    for line in file.lines() {
        let string = line.to_string();
        let direction = string.chars().next().unwrap();
        let distance = string[1..].parse::<i32>().unwrap();
        if direction == 'L' {
            let diff = dial - distance % 100;
            if diff < 0 {
                dial = 100 + diff;
            } else {
                dial = diff;
            }
        } else {
            dial = (dial + distance) % 100;
        }
        if dial == 0 {
            result += 1;
        }
    }

    println!("The result is {result}");
}
use std::fs::read_to_string;

pub fn main() {
    let file = read_to_string("input.txt").unwrap();
    let mut result = 0;
    let mut dial_index = 50usize;
    let mut dial = Vec::<u32>::with_capacity(100);

    for i in 0..100 {
        dial.push(i);
    }

    for line in file.lines() {
        let string = line.to_string();
        let direction = string.chars().next().unwrap();
        let distance = string[1..].parse::<usize>().unwrap();
        let mut i = 0usize;
        while i < distance {
            if direction == 'R' {
                dial_index = (dial_index + 1) % dial.len();
            } else {
                if dial_index == 0 {
                    dial_index = dial.len() - 1;
                } else {
                    dial_index = dial_index - 1;
                }
            }
            if dial[dial_index] == 0 {
                result += 1;
            }
            i += 1;
        }
    }

    println!("The result is {result}");
}
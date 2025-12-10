mod challenge_01_part_1;

use std::fs::{read_to_string};

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let mut dial = 50f32;
    let mut result = 0f32;

    for line in file.lines() {
        let string = line.to_string();
        let direction = string.chars().next().unwrap();
        let distance = string[1..].parse::<f32>().unwrap();
        if direction == 'L' {
            let diff = dial - distance;
            let change = dial - distance % 100.;
            if diff < 0. {
                println!("result is {result}");
                result += (diff / 100.).floor().abs();
                println!("calculation is ({diff} / 100.).floor().abs() = {}", (diff / 100.).floor().abs());
                println!("and now it is {result}");
            }
            if change < 0. {
                dial = 100. + change;
            } else {
                dial = change;
            }
        } else {
            let change = dial + distance;
            if change > 100. {
                result += (change / 100.).floor();
            }
            dial = change % 100.;
        }
        if dial == 0. {
            result += 1.;
        }
        println!("dial = {dial}");
    }

    println!("The result is {result}");
}

#![allow(dead_code)]

mod challenge_01_part_1;
mod challenge_01_part_2;
mod challenge_02_part_1;
mod challenge_02_part_2;
mod challenge_03_part_1;
mod challenge_03_part_2;
mod challenge_04_part_1;
mod challenge_04_part_2;

use std::fs::{read_to_string};

fn main() {
    let file = read_to_string("input.txt").unwrap();

    for line in file.lines() {

    }
}
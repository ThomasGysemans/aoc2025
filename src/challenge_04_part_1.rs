use std::fs::read_to_string;

pub fn main() {
    let file = read_to_string("input.txt").unwrap();
    let first_line = file.lines().next().unwrap();
    let grid_size = first_line.len();
    let capacity = file.lines().count() * grid_size;
    assert_eq!(first_line.len(), file.lines().count(), "Grid isn't a square");

    let mut result: u32 = 0;
    let mut map: Vec<bool> = Vec::with_capacity(capacity);

    for line in file.lines() {
        for char in line.chars() {
            if char == '@' {
                map.push(true);
            } else {
                map.push(false);
            }
        }
    }

    let max = grid_size - 1;

    for y in 0..grid_size {
        for x in 0..grid_size {
            let cell = map[y * grid_size + x];
            if cell {
                let nw = if y > 0 && x > 0 { map[(y - 1) * grid_size + (x - 1)] } else { false } as u8;
                let n = if y > 0 { map[(y - 1) * grid_size + x] } else { false } as u8;
                let ne = if y > 0 && x < max { map[(y - 1) * grid_size + (x + 1)] } else { false } as u8;
                let w = if x > 0 { map[y * grid_size + (x - 1)] } else { false } as u8;
                let e = if x < max { map[y * grid_size + (x + 1)] } else { false } as u8;
                let sw = if y < max && x > 0 { map[(y + 1) * grid_size + (x - 1)] } else { false } as u8;
                let s = if y < max { map[(y + 1) * grid_size + x] } else { false } as u8;
                let se = if y < max && x < max { map[(y + 1) * grid_size + (x + 1)] } else { false } as u8;
                let total = nw + n + ne + w + e + sw + s + se;
                if total < 4 {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}

pub fn visualise_grid(size: usize, grid: Vec<bool>) {
    for i in 0..size {
        for j in 0..size {
            let cell = grid[i * size + j];
            let char = if cell { '@' } else { '.' };
            print!("{}", char);
        }
        println!();
    }
}
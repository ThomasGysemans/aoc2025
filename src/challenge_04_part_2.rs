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

    loop {
        let all_coordinates = crate::detect_removable_cells(grid_size, &mut map);
        if all_coordinates.len() == 0 {
            break;
        }
        result += all_coordinates.len() as u32;
        for coordinates in all_coordinates {
            let x = coordinates[0];
            let y = coordinates[1];
            map[y * grid_size + x] = false;
        }
    }

    println!("{}", result);
}

pub fn detect_removable_cells(grid_size: usize, map: &mut Vec<bool>) -> Vec<[usize; 2]> {
    let max = grid_size - 1;
    let mut removable_cells: Vec<[usize; 2]> = Vec::new();
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
                    removable_cells.push([x, y]);
                }
            }
        }
    }
    removable_cells
}
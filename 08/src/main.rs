use std::collections::{BTreeMap, HashSet};

use aoclib::read_input;

fn main() {
    let input = read_input("input.txt");

    let grid = input.into_2d_chars();

    let mut antennas = BTreeMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != '.' {
                antennas.entry(cell).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    let mut antinode_positions = HashSet::new();

    for (_, positions) in antennas.iter() {
        // Look at any pair of positions
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in positions.iter().skip(i + 1) {
                let dx = pos2.0 as isize - pos1.0 as isize;
                let dy = pos2.1 as isize - pos1.1 as isize;

                let possible_antinodes = vec![
                    (pos1.0 as isize - dx, pos1.1 as isize - dy),
                    (pos2.0 as isize + dx, pos2.1 as isize + dy),
                ];

                for antinode in possible_antinodes {
                    if antinode.0 >= 0 && antinode.0 < grid[0].len() as isize && antinode.1 >= 0 && antinode.1 < grid.len() as isize {
                        antinode_positions.insert((antinode.0 as usize, antinode.1 as usize));
                    }
                }
            }
        }
    }

    println!("Result: {}", antinode_positions.len());

    // Part 2

    let mut antinode_positions = HashSet::new();

    for (_, positions) in antennas {
        // Look at any pair of positions
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in positions.iter().skip(i + 1) {
                let dx = pos2.0 as isize - pos1.0 as isize;
                let dy = pos2.1 as isize - pos1.1 as isize;

                let mut multiple = 0;
                loop {
                    let antinode = (pos2.0 as isize + dx * multiple, pos2.1 as isize + dy * multiple);
                    if antinode.0 >= 0 && antinode.0 < grid[0].len() as isize && antinode.1 >= 0 && antinode.1 < grid.len() as isize {
                        antinode_positions.insert((antinode.0 as usize, antinode.1 as usize));
                    } else {
                        break;
                    }
                    multiple += 1;
                }

                let mut multiple = 0;
                loop {
                    let antinode = (pos1.0 as isize - dx * multiple, pos1.1 as isize - dy * multiple);
                    if antinode.0 >= 0 && antinode.0 < grid[0].len() as isize && antinode.1 >= 0 && antinode.1 < grid.len() as isize {
                        antinode_positions.insert((antinode.0 as usize, antinode.1 as usize));
                    } else {
                        break;
                    }
                    multiple += 1;
                }
            }
        }
    }

    println!("Result2: {}", antinode_positions.len());
}

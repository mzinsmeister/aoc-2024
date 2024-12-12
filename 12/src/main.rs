use std::collections::HashMap;

use aoclib::read_input;

fn main() {
    let input = read_input("input.txt");

    let input = input.into_2d_chars();

    let mut explored_map = input.iter().map(|row| row.iter().map(|_| false).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();

    let mut result = 0;
    let mut result2 = 0;

    loop {

        let mut start_position = None;
        // Find the first unexplored cell
        for (y, row) in explored_map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if !cell {
                    start_position = Some((x, y));
                    break;
                }
            }
        }

        let start_position = match start_position {
            Some(position) => position,
            None => break,
        };

        let mut workset = vec![start_position];
        let mut fence_pieces = 0;
        let mut area = 0;
        let mut sides = 0;
        let current = input[start_position.1][start_position.0];

        while let Some((x, y)) = workset.pop() {
            if explored_map[y][x] {
                continue;
            }

            area += 1;
            if x == 0 || input[y][x - 1] != current {
                fence_pieces += 1;
                if y == 0 || input[y - 1][x] != current || (x > 0 && input[y - 1][x - 1] == current) {
                    sides += 1;
                }
            } else if !explored_map[y][x - 1] {
                workset.push((x - 1, y));
            }

            if x == input[y].len() - 1 || input[y][x + 1] != current {
                fence_pieces += 1;
                if y == 0 || input[y - 1][x] != current || (x < input[y].len() - 1 && input[y - 1][x + 1] == current) {
                    sides += 1;
                }
            } else if !explored_map[y][x + 1] {
                workset.push((x + 1, y));
            }

            if y == 0 || input[y - 1][x] != current {
                fence_pieces += 1;
                if x == 0 || input[y][x - 1] != current || (y > 0 && input[y - 1][x - 1] == current) {
                    sides += 1;
                }
            } else if !explored_map[y - 1][x] {
                workset.push((x, y - 1));
            }

            if y == input.len() - 1 || input[y + 1][x] != current {
                fence_pieces += 1;
                if x == 0 || input[y][x - 1] != current || (y < input.len() - 1 && input[y + 1][x - 1] == current) {
                    sides += 1;
                }
            } else if !explored_map[y + 1][x] {
                workset.push((x, y + 1));
            }

            explored_map[y][x] = true;
        }

        //println!("{} - Area: {}, Fence pieces: {}, Sides: {}", input[start_position.1][start_position.0], area, fence_pieces, sides);

        result += area * fence_pieces;
        result2 += area * sides;
    }

    println!("Result: {}", result);
    println!("Result2: {}", result2);
}

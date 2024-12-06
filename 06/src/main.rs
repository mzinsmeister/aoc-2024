use std::collections::{BTreeSet, HashSet};

use aoclib::read_input;

fn main() {
    let input = read_input("input.txt").into_2d_chars();

    let mut guard_position: Option<(usize, usize)> = None;

    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '^' {
                guard_position = Some((x, y));
                break;
            }
        }
    }

    let guard_starting_position = guard_position.unwrap();
    let mut guard_position = guard_starting_position;
    let mut guard_direction = (0, -1);

    let mut positions_visited = BTreeSet::new();

    loop {
        positions_visited.insert(guard_position);
        let ahead_position = (guard_position.0 as isize + guard_direction.0, guard_position.1 as isize + guard_direction.1);
        if ahead_position.0 < 0 || ahead_position.1 < 0 || ahead_position.0 >= input[0].len() as isize || ahead_position.1 >= input.len() as isize {
            break;
        }
        let ahead_position = (ahead_position.0 as usize, ahead_position.1 as usize);

        if input[ahead_position.1][ahead_position.0] == '#' {
            guard_direction = (-guard_direction.1, guard_direction.0);
        } else {
            guard_position = ahead_position;
        }
    }

    println!("Part 1: {}", positions_visited.len());

    // Try out adding a new obstacle for each visited position except the first one

    let mut loops_found = 0;

    for pos in positions_visited {
        let mut positions_visited = BTreeSet::new();
        let mut guard_position = guard_starting_position;
        let mut guard_direction = (0, -1);

        loop {
            if positions_visited.contains(&(guard_position, guard_direction)) {
                loops_found += 1;
                break;
            }
            positions_visited.insert((guard_position, guard_direction));
            let ahead_position = (guard_position.0 as isize + guard_direction.0, guard_position.1 as isize + guard_direction.1);
            if ahead_position.0 < 0 || ahead_position.1 < 0 || ahead_position.0 >= input[0].len() as isize || ahead_position.1 >= input.len() as isize {
                break;
            }
            let ahead_position = (ahead_position.0 as usize, ahead_position.1 as usize);

            if input[ahead_position.1][ahead_position.0] == '#' || ahead_position == pos {
                guard_direction = (-guard_direction.1, guard_direction.0);
            } else {
                guard_position = ahead_position;
            }
        }
    }

    println!("Part 2: {}", loops_found);
}

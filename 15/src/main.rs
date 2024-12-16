use std::collections::BTreeSet;

use aoclib::read_input;

fn main() {
    let input = read_input("input.txt");

    let (grid, path) = input.split_once("\n\n");

    let orig_grid = grid.into_2d_chars();
    let mut grid = orig_grid.clone();

    let path: Vec<(i32, i32)> = path.lines().map(|l| l.chars()).flatten().map(|c| {
        match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid character in path: {}", c),
        }
    }).collect();

    let mut start_pos = (0, 0);
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                start_pos = (x as i32, y as i32);
            }
        }
    }

    let mut pos = start_pos;

    for (dx, dy) in path.iter() {
        let new_pos = (pos.0 + dx, pos.1 + dy);

        let mut check_pos = new_pos;
        while grid[check_pos.1 as usize][check_pos.0 as usize] != '#' && grid[check_pos.1 as usize][check_pos.0 as usize] != '.' {
            check_pos = (check_pos.0 + dx, check_pos.1 + dy);
        }

        if grid[check_pos.1 as usize][check_pos.0 as usize] == '.' {
            if grid[new_pos.1 as usize][new_pos.0 as usize] == 'O' {
                grid[check_pos.1 as usize][check_pos.0 as usize] = 'O';
            }
            grid[new_pos.1 as usize][new_pos.0 as usize] = '@';
            grid[pos.1 as usize][pos.0 as usize] = '.';
            pos = new_pos;
        }

        /*for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                print!("{}", grid[y][x]);
            }
            println!("");
        }*/
    }

    let mut result = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
                result += y * 100 + x
            }
        }
    }

    println!("{}", result);

    let mut grid2 = vec![vec!['.'; orig_grid[0].len() * 2]; orig_grid.len()];

    for y in 0..orig_grid.len() {
        for x in 0..orig_grid[y].len(){
            if orig_grid[y][x] == 'O' {
                grid2[y][x*2] = '[';
                grid2[y][x*2+1] = ']';
            } else if orig_grid[y][x] == '@' {
                grid2[y][x*2] = '@';
            } else {
                grid2[y][x*2] = orig_grid[y][x];
                grid2[y][x*2+1] = orig_grid[y][x];
            }
        }
    }

    let mut grid = grid2;
    let mut pos = (start_pos.0 * 2, start_pos.1);

    'outer:
    for (dx, dy) in path {
        let new_pos = (pos.0 + dx, pos.1 + dy);

        if dy == 0 {
            let mut check_pos = new_pos;
            while grid[check_pos.1 as usize][check_pos.0 as usize] != '#' && grid[check_pos.1 as usize][check_pos.0 as usize] != '.' {
                check_pos = (check_pos.0 + dx, check_pos.1 + dy);
            }
    
            if grid[check_pos.1 as usize][check_pos.0 as usize] == '.' {
                let mut move_pos = (check_pos.0 - dx, check_pos.1 - dy);
                if grid[move_pos.1 as usize][move_pos.0 as usize] == '[' || grid[move_pos.1 as usize][move_pos.0 as usize] == ']' {
                    while move_pos != pos {
                        let prev_pos = (move_pos.0 + dx, move_pos.1 + dy);
                        grid[prev_pos.1 as usize][prev_pos.0 as usize] = grid[move_pos.1 as usize][move_pos.0 as usize];
                        move_pos = (move_pos.0 - dx, move_pos.1 - dy);
                    }
                }
                grid[new_pos.1 as usize][new_pos.0 as usize] = '@';
                grid[pos.1 as usize][pos.0 as usize] = '.';
                pos = new_pos;
            }
        } else {
            let mut new_grid = grid.clone();

            let mut boxes_to_move = BTreeSet::new();
            let mut check_pos: Vec<(i32, i32)> = Vec::new();
            check_pos.push(new_pos);
            while let Some((x, y)) = check_pos.pop() {
                if grid[y as usize][x as usize] == '#' {
                    continue 'outer;
                }
                if grid[y as usize][x as usize] == '[' {
                    boxes_to_move.insert((x, y));
                    check_pos.push((x, y + dy));
                    check_pos.push((x + 1, y + dy));
                }
                if grid[y as usize][x as usize] == ']' {
                    boxes_to_move.insert((x - 1, y));
                    check_pos.push((x - 1, y + dy));
                    check_pos.push((x, y + dy));
                }
            }

            for (x, y) in boxes_to_move.iter() {
                new_grid[*y as usize][*x as usize] = '.';
                new_grid[*y as usize][(x + 1) as usize] = '.';
            }

            for (x, y) in boxes_to_move {
                new_grid[(y + dy) as usize][x as usize] = '[';
                new_grid[(y + dy) as usize][(x + 1) as usize] = ']';
            }

            new_grid[new_pos.1 as usize][new_pos.0 as usize] = '@';
            new_grid[pos.1 as usize][pos.0 as usize] = '.';
            
            grid = new_grid;
            pos = new_pos;
        }



        /*for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                print!("{}", grid[y][x]);
            }
            println!("");
        }*/
    }

    let mut result = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '[' {
                result += y * 100 + x
            }
        }
    }

    println!("{}", result);

}

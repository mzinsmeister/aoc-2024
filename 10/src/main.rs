use std::collections::BTreeSet;

use aoclib::read_input;

fn main() {
    let input = read_input("input.txt");

    let grid = input.into_2d_chars();

    let nr_grid = grid.iter()
        .map(|l| l.iter().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut trailheads = vec![];

    for (y, row) in nr_grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 0 {
                trailheads.push((x, y));
            }
        }
    }

    let mut result = 0;
    let mut result2 = 0;

    for (x, y) in trailheads {
        let mut workset = Vec::new();
        workset.push((x, y));

        let mut endpoints = BTreeSet::new();


        while let Some((x, y)) = workset.pop() {
            if nr_grid[y][x] == 9 {
                endpoints.insert((x, y));
                result2 += 1;
                continue;
            }
            let mut candidates = vec![];
            if x > 0 {
                candidates.push((x - 1, y));
            }
            if x < nr_grid[0].len() - 1 {
                candidates.push((x + 1, y));
            }
            if y > 0 {
                candidates.push((x, y - 1));
            }
            if y < nr_grid.len() - 1 {
                candidates.push((x, y + 1));
            }
            for c in candidates {
                if nr_grid[c.1][c.0] == nr_grid[y][x] + 1 {
                    workset.push(c);
                }
            }
        }

        result += endpoints.len();
    }

    println!("Result: {}", result);

    println!("Result2: {}", result2);
}

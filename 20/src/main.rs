use std::collections::VecDeque;

use aoclib::read_input;

fn get_dist_matrix(track: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Vec<usize>> {
    let mut workset = VecDeque::new();

    workset.push_back((start.0, start.1, 0));

    let mut dist = vec![vec![usize::MAX; track[0].len()]; track.len()];

    while let Some((x, y, cost)) = workset.pop_back() {
        if dist[y][x] <= cost {
            continue;
        }

        let cell = track[y][x];
        if cell == '#' {
            continue;
        }

        dist[y][x] = cost;

        if x > 0 {
            workset.push_back((x - 1, y, cost + 1));
        }

        if y > 0 {
            workset.push_back((x, y - 1, cost + 1));
        }

        if x < track[0].len() - 1 {
            workset.push_back((x + 1, y, cost + 1));
        }

        if y < track.len() - 1 {
            workset.push_back((x, y + 1, cost + 1));
        }
    }

    dist
}

fn main() {
    let input = read_input("input.txt");

    let track = input.into_2d_chars();

    let mut workset = VecDeque::new();

    for (y, row) in track.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                workset.push_back((x, y, 0));
            }
        }
    }

    let mut path_positions = vec![];

    for (y, row) in track.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '#' && x > 0 && y > 0 && x < track[0].len() - 1 && y < track.len() - 1 {
                path_positions.push((x, y));
            }
        }
    }

    let mut start_pos = (0,0);
    let mut end_pos = (0,0);

    for (x, y) in path_positions.iter() {
        if track[*y][*x] == 'S' {
            start_pos = (*x, *y);
        }
        if track[*y][*x] == 'E' {
            end_pos = (*x, *y);
        }
    }

    let start_dist = get_dist_matrix(&track, start_pos);
    let end_dist = get_dist_matrix(&track, end_pos);

    let no_cheat_length = start_dist[end_pos.1][end_pos.0];

    let mut result = 0;

    for cheat_pos in path_positions.iter() {
        for cheat_pos_end in path_positions.iter() {
            let cheat_dist = cheat_pos.0.abs_diff(cheat_pos_end.0) + cheat_pos.1.abs_diff(cheat_pos_end.1);
            if cheat_dist <= 2 {
                let cheat_start_dist = start_dist[cheat_pos.1][cheat_pos.0];
                let cheat_end_dist = end_dist[cheat_pos_end.1][cheat_pos_end.0];
                let cheat_length = cheat_start_dist + cheat_end_dist + cheat_dist;
                if cheat_length + 100 <= no_cheat_length {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);


    let mut result = 0;

    for cheat_pos in path_positions.iter() {
        for cheat_pos_end in path_positions.iter() {
            let cheat_dist = cheat_pos.0.abs_diff(cheat_pos_end.0) + cheat_pos.1.abs_diff(cheat_pos_end.1);
            if cheat_dist <= 20 {
                let cheat_start_dist = start_dist[cheat_pos.1][cheat_pos.0];
                let cheat_end_dist = end_dist[cheat_pos_end.1][cheat_pos_end.0];
                let cheat_length = cheat_start_dist + cheat_end_dist + cheat_dist;
                if cheat_length + 100 <= no_cheat_length {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);

}

use std::collections::{BTreeMap, VecDeque};

use aoclib::{read_input, Direction};

fn find_all_shortest_paths(numeric_connected: &BTreeMap<char, Vec<(char, Direction)>>, start: char, end: char) -> Vec<Vec<Direction>> {
    let mut paths: Vec<Vec<Direction>> = Vec::new();

    let mut queue = VecDeque::new();

    queue.push_back((start, Vec::new()));

    while !queue.is_empty() {
        let (current, path) = queue.pop_front().unwrap();

        if paths.len() > 0 && path.len() > paths[0].len() {
            continue;
        }
        
        if current == end {
            paths.push(path);
            continue;
        }

        for (next, direction) in numeric_connected.get(&current).unwrap() {
            let mut new_path = path.clone();
            new_path.push(*direction);
            queue.push_back((*next, new_path));
        }
    }

    paths
}

// We -> R1 -> R2 -> R3 -> num

fn main() {
    let input = read_input("input.txt");

    let lines: Vec<&str> = input.trim_last_newline().lines().collect();

    let mut numeric_connected = BTreeMap::new();

    numeric_connected.insert('7', vec![('8', Direction::Right), ('4', Direction::Down)]);
    numeric_connected.insert('8', vec![('9', Direction::Right), ('5', Direction::Down), ('7', Direction::Left)]);
    numeric_connected.insert('9', vec![('6', Direction::Down), ('8', Direction::Left)]);
    numeric_connected.insert('4', vec![('5', Direction::Right), ('1', Direction::Down), ('7', Direction::Up)]);
    numeric_connected.insert('5', vec![('6', Direction::Right), ('2', Direction::Down), ('4', Direction::Left), ('8', Direction::Up)]);
    numeric_connected.insert('6', vec![('3', Direction::Down), ('5', Direction::Left), ('9', Direction::Up)]);
    numeric_connected.insert('1', vec![('2', Direction::Right), ('4', Direction::Up)]);
    numeric_connected.insert('2', vec![('3', Direction::Right), ('5', Direction::Up), ('1', Direction::Left), ('0', Direction::Down)]);
    numeric_connected.insert('3', vec![('6', Direction::Up), ('2', Direction::Left), ('A', Direction::Down)]);
    numeric_connected.insert('0', vec![('2', Direction::Up), ('A', Direction::Right)]);
    numeric_connected.insert('A', vec![('3', Direction::Up), ('0', Direction::Left)]);


    let mut directions_connected = BTreeMap::new();

    directions_connected.insert('^', vec![('v', Direction::Down), ('A', Direction::Right)]);
    directions_connected.insert('v', vec![('^', Direction::Up), ('<', Direction::Left), ('>', Direction::Right)]);
    directions_connected.insert('<', vec![('v', Direction::Right)]);
    directions_connected.insert('>', vec![('v', Direction::Left), ('A', Direction::Up)]);
    directions_connected.insert('A', vec![('^', Direction::Left), ('>', Direction::Down)]);

    let numerics = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'];
    let directions = vec!['^', 'v', '<', '>', 'A'];

    // We now precalculate the shortest paths on the highest level between all
    // numerics. This works because all the direction keyboards will be on 'A' after typing a numeric.

    // What we must press so that R1 moves on R2's pad
    let mut r1_shortest_paths: BTreeMap<(char, char), usize> = BTreeMap::new();

    for start_dir in directions.iter() {
        for end_dir in directions.iter() {
            let paths = find_all_shortest_paths(&directions_connected, *start_dir, *end_dir);

            r1_shortest_paths.insert((*start_dir, *end_dir), paths[0].len() + 1);
        }
    }

    // What we must press so that R2 moves on R3's pad
    let mut r2_shortest_paths: BTreeMap<(char, char), usize> = BTreeMap::new();

    for start_dir in directions.iter() {
        for end_dir in directions.iter() {
            let paths = find_all_shortest_paths(&directions_connected, *start_dir, *end_dir);

            let mut min = std::usize::MAX;

            for path in paths {
                let mut sum = 0;
                let mut current = 'A';

                for dir in path {
                    sum += r1_shortest_paths.get(&(current, dir.into_char())).unwrap();
                    current = dir.into_char();
                }

                sum += r1_shortest_paths.get(&(current, 'A')).unwrap();

                if sum < min {
                    min = sum;
                }
            }

            r2_shortest_paths.insert((*start_dir, *end_dir), min);
        }
    }

    // What we must press so that R3 moves on the numeric pad
    let mut r3_shortest_paths: BTreeMap<(char, char), usize> = BTreeMap::new();

    for start_n in numerics.iter() {
        for end_n in numerics.iter() {
            let paths = find_all_shortest_paths(&numeric_connected, *start_n, *end_n);

            let mut min = std::usize::MAX;

            for path in paths {
                let mut sum = 0;
                let mut current = 'A';

                for dir in path {
                    sum += r2_shortest_paths.get(&(current, dir.into_char())).unwrap();
                    current = dir.into_char();
                }

                sum += r2_shortest_paths.get(&(current, 'A')).unwrap();

                if sum < min {
                    min = sum;
                }
            }

            r3_shortest_paths.insert((*start_n, *end_n), min);
        }
    }

    let mut result = 0;

    for line in lines.iter() {

        let mut sum = 0;

        let mut current = 'A';

        for c in line.chars() {
            sum += r3_shortest_paths.get(&(current, c)).unwrap();
            current = c;
        }

        let numeric_part = line[0..line.len() - 1].parse::<usize>().unwrap();

        result += sum * numeric_part;
    }

    println!("{}", result);

    let mut rs_shortest_paths = vec![r1_shortest_paths];

    for _ in 0..24 {
        let mut new_rs_shortest_paths = BTreeMap::new();

        for start_dir in directions.iter() {
            for end_dir in directions.iter() {
                let paths = find_all_shortest_paths(&directions_connected, *start_dir, *end_dir);

                let mut min = std::usize::MAX;

                for path in paths {
                    let mut sum = 0;
                    let mut current = 'A';

                    for dir in path {
                        sum += rs_shortest_paths[rs_shortest_paths.len() - 1].get(&(current, dir.into_char())).unwrap();
                        current = dir.into_char();
                    }

                    sum += rs_shortest_paths[rs_shortest_paths.len() - 1].get(&(current, 'A')).unwrap();

                    if sum < min {
                        min = sum;
                    }
                }

                new_rs_shortest_paths.insert((*start_dir, *end_dir), min);
            }
        }

        rs_shortest_paths.push(new_rs_shortest_paths);
    }

    let mut r_num_shortest_paths = BTreeMap::new();

    for start_n in numerics.iter() {
        for end_n in numerics.iter() {
            let paths = find_all_shortest_paths(&numeric_connected, *start_n, *end_n);

            let mut min = std::usize::MAX;

            for path in paths {
                let mut sum = 0;
                let mut current = 'A';

                for dir in path {
                    sum += rs_shortest_paths[rs_shortest_paths.len() - 1].get(&(current, dir.into_char())).unwrap();
                    current = dir.into_char();
                }

                sum += rs_shortest_paths[rs_shortest_paths.len() - 1].get(&(current, 'A')).unwrap();

                if sum < min {
                    min = sum;
                }
            }

            r_num_shortest_paths.insert((*start_n, *end_n), min);
        }
    }

    let mut result = 0;

    for line in lines.iter() {

        let mut sum = 0;

        let mut current = 'A';

        for c in line.chars() {
            sum += r_num_shortest_paths.get(&(current, c)).unwrap();
            current = c;
        }

        let numeric_part = line[0..line.len() - 1].parse::<usize>().unwrap();

        result += sum * numeric_part;
    }

    println!("{}", result);
}

use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet, BinaryHeap}};

use aoclib::{read_input, Direction};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize, Direction),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Modified Dijkstra from the Binary Heap doc.rust-lang.org example

fn shortest_path(input: &Vec<Vec<char>>, start: (usize, usize, Direction)) -> Vec<Vec<BTreeMap<Direction, usize>>> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<Vec<BTreeMap<Direction, usize>>> = vec![vec![BTreeMap::new(); input[0].len()]; input.len()];

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                dist[y][x].insert(dir, usize::MAX);
            }
        }
    }

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.1][start.0].insert(start.2, 0);
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if input[position.1][position.0] == 'E' { continue; }

        // Important as we may have already found a better way
        if cost > dist[position.1][position.0][&position.2] { continue; }


        let mut reachable_neighbors = vec![];

        let (dx, dy) = position.2.into_dxdy();

        if input[(position.1 as isize + dy) as usize][(position.0 as isize + dx) as usize] != '#' {
            reachable_neighbors.push((((position.0 as isize + dx) as usize, (position.1 as isize + dy) as usize, position.2), 1));
        }

        reachable_neighbors.push(((position.0, position.1, position.2.rotate_clockwise()), 1000));
        reachable_neighbors.push(((position.0, position.1, position.2.rotate_counter_clockwise()), 1000));

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (next_position, additional_cost) in reachable_neighbors {
            let next = State { cost: cost + additional_cost, position: next_position };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position.1][next.position.0][&next.position.2] {
                heap.push(next);
                // Relaxation, we have now found a better way
                *dist[next.position.1][next.position.0].get_mut(&next.position.2).unwrap() = next.cost;
            }
        }
    }

    return dist;
}

fn main() {
    let input = read_input("input.txt");

    let maze = input.into_2d_chars();

    let start_pos = maze
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &c)| {
                if c == 'S' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let end_pos = maze
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &c)| {
                if c == 'E' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();
        
    let dist = shortest_path(&maze, (start_pos.0, start_pos.1, Direction::Right));

    let result = dist[end_pos.1][end_pos.0].iter().min_by_key(|(_, &cost)| cost).unwrap().1;

    println!("Result: {}", result);

    // Because of the way our Graph is implicitly defined and the way we track cost for Dijkstra, 
    // we can now find all shortest paths backwards from the end position

    let mut shortest_path_fields: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut explore_set: BTreeSet<(usize, usize, Direction)> = BTreeSet::new();

    for (dir, cost) in dist[end_pos.1][end_pos.0].iter() {
        if cost == result {
            explore_set.insert((end_pos.0, end_pos.1, *dir));
        }
    }

    while let Some((x, y, dir)) = explore_set.pop_first() {
        shortest_path_fields.insert((x, y));

        let cur = dist[y][x][&dir];

        if cur == 0 {
            continue;
        }

        let (dx, dy) = dir.into_dxdy();

        let straight_prev = ((x as isize - dx) as usize, (y as isize - dy) as usize, dir);
        let ccw_prev = (x, y, dir.rotate_clockwise());
        let cw_prev = (x, y, dir.rotate_counter_clockwise());

        if dist[straight_prev.1][straight_prev.0][&straight_prev.2] == cur - 1 {
            explore_set.insert(straight_prev);
        }

        if dist[ccw_prev.1][ccw_prev.0][&ccw_prev.2] == cur - 1000 {
            explore_set.insert(ccw_prev);
        }

        if dist[cw_prev.1][cw_prev.0][&cw_prev.2] == cur - 1000 {
            explore_set.insert(cw_prev);
        }
    }

    println!("Result 2: {}", shortest_path_fields.len());

}

use std::{cmp::Ordering, collections::{BTreeSet, BinaryHeap}, usize};

use aoclib::{read_input, Direction};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
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

fn shortest_path(bounds: (usize, usize), corrupted: BTreeSet<(usize, usize)>) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; bounds.0 + 1]; bounds.1 + 1];

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[0][0] = 0;
    heap.push(State { cost: 0, position: (0,0) });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == (bounds.0, bounds.1) {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position.1][position.0] { continue; }


        let mut reachable_neighbors = Vec::with_capacity(4);


        for dir in [Direction::Down, Direction::Up, Direction::Left, Direction::Right].iter() {
            let (dx, dy) = dir.into_dxdy();

            let new_position = (position.0 as isize + dx, position.1 as isize + dy);

            if new_position.0 < 0 || new_position.1 < 0 
                || new_position.0 > bounds.0 as isize 
                || new_position.1 > bounds.1 as isize
                || corrupted.contains(&(new_position.0 as usize, new_position.1 as usize)) {
                continue;
            }

            reachable_neighbors.push((new_position.0 as usize, new_position.1 as usize));
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for next_position in reachable_neighbors {
            let next = State { cost: cost + 1, position: next_position };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position.1][next.position.0] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.1][next.position.0] = next.cost;
            }
        }
    }

    return None
}

fn main() {
    let input = read_input("input.txt");

    let input = input.trim_last_newline().lines()
    .map(|l| {
        let (x,y) = l.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    })
    .collect::<Vec<(usize, usize)>>();

    let mut corrupted = BTreeSet::new();

    let bytes = 1024;

    for l in input.iter().take(bytes) {
        corrupted.insert(*l);
    }

    let bounds = (70, 70);

    let result = shortest_path(bounds, corrupted).unwrap();

    println!("Result: {}", result);

    // Binary search the space between 1024 and length of input

    let mut low = bytes;
    let mut high = input.len();

    while low < high {
        let mid = (low + high) / 2;

        let mut corrupted = BTreeSet::new();

        for l in input.iter().take(mid) {
            corrupted.insert(*l);
        }

        if let Some(_) = shortest_path(bounds, corrupted) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    println!("Result 2: {},{}", input[low-1].0, input[low-1].1);
}

use std::collections::{BTreeMap, BTreeSet};

use aoclib::read_input;

fn solve2(patterns: &Vec<Vec<char>>, towel: &Vec<char>, i: usize, mem_table: &mut BTreeMap<usize, usize>) -> usize {
    if i == towel.len() {
        return 1;
    }

    if let Some(&result) = mem_table.get(&i) {
        return result;
    }

    let mut result = 0;

    let remaining = &towel[i..];

    for p in patterns.iter() {
        if p.len() > remaining.len() {
            continue;
        }

        if p.as_slice() == &remaining[..p.len()] {
            let new_index = i + p.len();
            result += solve2(patterns, towel, new_index, mem_table);
        }
    }

    mem_table.insert(i, result);

    result
}

fn main() {
    let input = read_input("input.txt");

    let (patterns, towels) = input.split_once("\n\n");

    let patterns: Vec<Vec<char>> = patterns
        .split(", ")
        .map(|line| line.chars().collect())
        .collect();

    let towels: Vec<Vec<char>> = towels.lines().map(|line| line.chars().collect()).collect();

    let mut result = 0;

    'outer:
    for t in towels.iter() {

        let mut seen = BTreeSet::new();
        let mut workset = vec![0usize];

        while let Some(i) = workset.pop() {
            seen.insert(i);
            let remaining = &t[i..];

            for p in patterns.iter() {
                if p.len() > remaining.len() {
                    continue;
                }

                if p.as_slice() == &remaining[..p.len()] {
                    let new_index = i + p.len();
                    if new_index == t.len() {
                        result += 1;
                        continue 'outer;
                    } else if !seen.contains(&new_index) {
                        workset.push(new_index);
                    }
                }
            }
        }

    }

    println!("{}", result);


    let mut result = 0;

    for t in towels {

        let mut mem_table = BTreeMap::new();

        result += solve2(&patterns, &t, 0, &mut mem_table);

    }

    println!("{}", result);

}

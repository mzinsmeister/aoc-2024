use std::{collections::HashMap, fs::read_to_string, ops::DerefMut};

use aoclib::read_input;

fn main() {
    let input = read_input("input.txt");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for l in input.lines() {
        let (left, right) = l.split_once("   ").unwrap();
        list1.push(left.parse::<u32>().unwrap());
        list2.push(right.parse::<u32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let result: u32 = list1.iter().zip(list2.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum();

    println!("Part 1 Result: {}", result);

    let mut right_index = HashMap::<u32,u32>::new();
    list2.iter().for_each(|e| *right_index.entry(*e).or_default() += 1);

    let result2: u32 = list1.iter()
        .map(|l| right_index.get(l).cloned().unwrap_or_default() * *l)
        .sum();

    println!("Part 2 Result: {}", result2);

}

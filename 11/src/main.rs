use std::collections::HashMap;

use aoclib::read_input;

#[derive(Debug, Copy, Clone)]
struct Stone {
    value: u64,
    next: Option<usize>    
}

fn simulate_stones(memo: &mut HashMap<(u64, usize), usize>, nr: u64, number_blinks_left: usize) -> u64 {
    if number_blinks_left == 0 {
        return 1;
    }

    if let Some(&result) = memo.get(&(nr, number_blinks_left)) {
        return result as u64;
    }

    let result = if nr == 0 {
        simulate_stones(memo, 1, number_blinks_left - 1)
    } else {
        let stone_str = nr.to_string();
        if stone_str.len() % 2 == 0 {
            let first_half = stone_str[..stone_str.len() / 2].parse::<u64>().unwrap();
            let second_half = stone_str[stone_str.len() / 2..].parse::<u64>().unwrap();
            simulate_stones(memo, first_half, number_blinks_left - 1) + simulate_stones(memo, second_half, number_blinks_left - 1)
        } else {
            simulate_stones(memo, nr * 2024, number_blinks_left - 1)
        }
    };

    memo.insert((nr, number_blinks_left), result as usize);

    result
}

fn main() {
    let input = read_input("input.txt");

    let input = input.trim_last_newline().split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut stones = Vec::new();

    for i in 0..input.len() {
        stones.push(Stone { value: input[i], next: Some(i + 1) });
    }

    stones.last_mut().unwrap().next = None;

    // Naive brute force solution for part 1

    for _ in 0..25 {
        let mut i: usize = 0;
        loop {
            let stone = stones[i].value;
            let next = stones[i].next;
    
            //print!("{} ", stone);
    
            if stone == 0 {
                stones[i].value = 1;
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let first_half = stone_str[..stone_str.len() / 2].parse::<u64>().unwrap();
                    let second_half = stone_str[stone_str.len() / 2..].parse::<u64>().unwrap();
                    stones.push(Stone { value: second_half, next: stones[i].next });
                    stones[i].value = first_half;                
                    stones[i].next = Some(stones.len() - 1);
                } else {
                    stones[i].value *= 2024;
                }
            }

            if let Some(next) = next {
                i = next;
            } else {
                break;
            }
        }
        //println!(" ");
    }

    println!("{}", stones.len());

    // We can simulate each stone independently -> Smart Memoization solution

    let mut memo: HashMap<(u64, usize), usize> = HashMap::new();

    let mut result = 0;

    for stone in input {
        result += simulate_stones(&mut memo, stone, 75);
    }

    println!("{}", result);
}

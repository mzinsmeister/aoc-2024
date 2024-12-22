use std::collections::VecDeque;

use aoclib::read_input;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn get_next_nr(mut nr: u64) -> u64 {
    nr = (nr ^ (nr * 64)) % 16777216;
    nr = (nr ^ (nr / 32)) % 16777216;
    nr = (nr ^ (nr * 2048)) % 16777216;
    nr
}

fn main() {
    let input = read_input("input.txt");

    let initials = input.lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u64>>();


    let mut sum = 0;

    for &nr in initials.iter() {
        let mut nr = nr;
        for _ in 0..2000 {
            nr = get_next_nr(nr);
        }
        sum += nr;
    }

    println!("{}", sum);

    let max = (-9..=9).into_par_iter().map(|nr1| {
        let mut max = 0;
        for nr2 in 1..=1 {
            for nr3 in -9..=9 {
                for nr4 in -9..=9 {
                    let sequence = [nr1, nr2, nr3, nr4];
                    let mut sum = 0;
                    for &start_nr in initials.iter() {
                        let mut nr = start_nr;
                        let mut history = VecDeque::new();
                        'numbers:
                        for _ in 0..2000 {
                            let next_nr = get_next_nr(nr);
                            let diff = (next_nr % 10) as i64 - (nr % 10) as i64;
                            if history.len() == 4 {
                                history.pop_front();
                            }
                            history.push_back(diff);
                            nr = next_nr;

                            if history.len() < 4 {
                                continue;
                            }
                            for i in 0..4 {
                                if history[i] != sequence[i] {
                                    continue 'numbers;
                                }
                            }
                            sum += nr % 10;
                            break;
                        }
                    }
                    if sum > max {
                        max = sum;
                    }    
                }
            }
        }
        max
    }).max().unwrap();

    println!("{}", max);
    
}

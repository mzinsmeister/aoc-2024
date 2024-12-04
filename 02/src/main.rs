use aoclib::read_input;

fn check_report(r: &[i32]) -> bool {
    let mut current = r[0];
    if r[0] < r[1] {
        // Increasing
        for i in 1..r.len() {
            let diff = r[i] - current;
            if diff < 1 || diff > 3 {
                return false;
            }
            current = r[i];
        }
    } else if r[0] > r[1] {
        // Decreasing
        for i in 1..r.len() {
            let diff = current - r[i];
            if diff < 1 || diff > 3 {
                return false;
            }
            current = r[i];
        }
    } else {
        return false;
    }
    true
}

fn main() {
    let input = read_input("input.txt");

    let reports: Vec<Vec<i32>> = input.split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();

    let result1 = reports.iter().filter(|r| check_report(r)).count();
    let mut result2 = 0;

    // Brute force
    // There's an O(n) solution to be had here but i spent too much time trying to make it work
    // General Idea: Have a "joker" that you set if you skip a number. First number needs special handling

    'outer:
    for r in reports.iter() {
        for remove in 0..r.len() {
            let mut r = r.clone();
            r.remove(remove);
            if check_report(&r) {
                result2 += 1;
                continue 'outer;
            }
        }
    }

    
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}

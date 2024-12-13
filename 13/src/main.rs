use std::{str::FromStr, usize};

use aoclib::read_input;

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    goal: (usize, usize)
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a_line = lines.next().unwrap();
        let b_line = lines.next().unwrap();
        let goal_line = lines.next().unwrap();

        let (a_x_raw, a_y_raw) = a_line[12..a_line.len()].split_once(", Y+").unwrap();
        let (b_x_raw, b_y_raw) = b_line[12..b_line.len()].split_once(", Y+").unwrap();
        let (goal_x_raw, goal_y_raw) = goal_line[9..goal_line.len()].split_once(", Y=").unwrap();

        let a = (a_x_raw.parse().unwrap(), a_y_raw.parse().unwrap());
        let b = (b_x_raw.parse().unwrap(), b_y_raw.parse().unwrap());
        let goal = (goal_x_raw.parse().unwrap(), goal_y_raw.parse().unwrap());

        Ok(Machine { a, b, goal })
    }
}

fn main() {
    let input = read_input("input.txt");

    let input: Vec<Machine> = input.trim_last_newline()
        .split("\n\n").map(|m| m.parse().unwrap())
        .collect();

    let mut result = 0;


    for machine in input.iter() {

        if machine.a.0 / machine.b.0 == machine.a.1 / machine.b.1 && machine.goal.0 % machine.a.0 == 0 && machine.goal.1 % machine.a.1 == 0 {
            panic!("infinite solutions");
        }

        /*
            Solve system of equations:
            a_x * a + b_x * b = goal_x
            a_y * a + b_y * b = goal_y

            a = (b_y * goal_x - b_x * goal_y) / (b_y * a_x - b_x * a_y)
            b = (goal_x - a_x * a) / b_x
         */

        let a_x = machine.a.0 as isize;
        let a_y = machine.a.1 as isize;
        let b_x = machine.b.0 as isize;
        let b_y = machine.b.1 as isize;
        let goal_x = machine.goal.0 as isize;
        let goal_y = machine.goal.1 as isize;     

        let a = (b_y * goal_x - b_x * goal_y) / (b_y * a_x - b_x * a_y);

        if a < 0 || (b_y * goal_x - b_x * goal_y) % (b_y * a_x - b_x * a_y) != 0 {
            continue;
        }

        let b = (goal_x - a_x * a) / b_x;

        if b < 0 || (goal_x - a_x * a) % b_x != 0 {
            continue;
        }        


        //println!("{:?} a: {}, b: {}", machine, a, b);

        result += 3*a + b;
    }

    println!("{}", result);

    let mut result = 0;


    for machine in input.iter() {

        if machine.a.0 / machine.b.0 == machine.a.1 / machine.b.1 && machine.goal.0 % machine.a.0 == 0 && machine.goal.1 % machine.a.1 == 0 {
            panic!("infinite solutions");
        }

        /*
            Solve system of equations:
            a_x * a + b_x * b = goal_x
            a_y * a + b_y * b = goal_y

            a = (b_y * goal_x - b_x * goal_y) / (b_y * a_x - b_x * a_y)
            b = (goal_x - a_x * a) / b_x
         */

        let a_x = machine.a.0 as isize;
        let a_y = machine.a.1 as isize;
        let b_x = machine.b.0 as isize;
        let b_y = machine.b.1 as isize;
        let goal_x = machine.goal.0 as isize + 10000000000000;
        let goal_y = machine.goal.1 as isize + 10000000000000;     

        let a = (b_y * goal_x - b_x * goal_y) / (b_y * a_x - b_x * a_y);

        if a < 0 || (b_y * goal_x - b_x * goal_y) % (b_y * a_x - b_x * a_y) != 0 {
            continue;
        }

        let b = (goal_x - a_x * a) / b_x;

        if b < 0 || (goal_x - a_x * a) % b_x != 0 {
            continue;
        }        


        //println!("{:?} a: {}, b: {}", machine, a, b);

        result += 3*a + b;
    }
    println!("{}", result);
}

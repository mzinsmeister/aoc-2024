use std::str::FromStr;

use aoclib::read_input;

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: (i32, i32),
    d: (i32, i32),
}

impl Robot {
    fn step(&mut self, bounds: (i32, i32)) {
        self.p.0 += self.d.0;
        self.p.1 += self.d.1;

        self.p.0 = self.p.0.rem_euclid(bounds.0);
        self.p.1 = self.p.1.rem_euclid(bounds.1);
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p_raw, v_raw) = s[2..s.len()].split_once(" v=").unwrap();

        let p = p_raw.split_once(",").unwrap();
        let v = v_raw.split_once(",").unwrap();

        Ok(Robot {
            p: (p.0.parse().unwrap(), p.1.parse().unwrap()),
            d: (v.0.parse().unwrap(), v.1.parse().unwrap()),
        })
    }

}

fn main() {
    let input = read_input("input.txt");
    //let bounds = (11, 7);
    let bounds = (101, 103);

    let mut robots: Vec<Robot> = input.lines().map(|l| l.parse().unwrap()).collect();

    for _t in 0..100 {
        for r in robots.iter_mut() {
            r.step(bounds);
        }
    }

    let mut quadrants = [[0,0],[0,0]];

    for r in robots.iter() {
        if r.p.0 == bounds.0 / 2 || r.p.1 == bounds.1 / 2 {
            continue;
        }
        if r.p.0 < bounds.0 / 2 {
            if r.p.1 < bounds.1 / 2 {
                quadrants[0][0] += 1;
            } else {
                quadrants[0][1] += 1;
            }
        } else {
            if r.p.1 < bounds.1 / 2 {
                quadrants[1][0] += 1;
            } else {
                quadrants[1][1] += 1;
            }
        }
    }

    let result = quadrants[0][0] * quadrants[1][1] * quadrants[0][1] * quadrants[1][0];

    println!("Result: {}", result);

    let mut robots = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<Robot>>();

    // Solution seems to be first time no two robots are in one place...
    // Thanks Reddit... 
    // Would have been nice if the problem specified what the frikkin christmas tree 
    // we were searching for looked like...

    let mut t = 0;
    loop {
        t += 1;
        let mut grid = vec![vec![0; bounds.0 as usize]; bounds.1 as usize];
        let mut print = true;
        for r in robots.iter_mut() {
            r.step(bounds);
            grid[r.p.1 as usize][r.p.0 as usize] += 1;
            if grid[r.p.1 as usize][r.p.0 as usize] > 1 {
                print = false;
            }
        }

        if print {
            println!("{}", t);
            for l in grid.iter() {
                for c in l.iter() {
                    if *c == 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
                println!("");
            }
            println!("\n");

            break;
        }
    }
}

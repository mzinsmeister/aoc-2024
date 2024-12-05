use std::collections::BTreeMap;

use aoclib::read_input;

fn main() {
    let input = read_input("input.txt");

    let (rules, lists) = input.trim_last_newline().split_once("\n\n").unwrap();

    let rules = rules.lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(l, r)| {
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        });

    let mut rules_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();


    for (l, r) in rules {
        rules_map.entry(r).or_default().push(l);
    }

    let lists = lists.lines()
            .map(|l| l.split(",").map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>());

    let mut result = 0;

    let mut incorrect_lines = Vec::new();

    'outer:
    for l in lists {
        for (i, n) in l.iter().enumerate() {
            let rules = rules_map.get(n);

            if let Some(rules) = rules {
                for n2 in l.iter().skip(i + 1) {
                    if rules.contains(&n2) {
                        incorrect_lines.push(l);
                        continue 'outer;
                    }
                }
            }            
        }
        result += l[l.len() / 2];
    }

    println!("{}", result);

    let mut result2 = 0;

    for l in incorrect_lines.iter_mut() {
        let mut change = true;
        while change {
            change = false;
            let mut i = 0;
            while i < l.len() {
                let n = l[i];
                let rules = rules_map.get(&n);
    
                if let Some(rules) = rules {
                    for i2 in i+1..l.len() {
                        let n2 = l[i2];
                        if rules.contains(&n2) {
                            // Move the number in question before our current number
                            l.remove(i2);
                            l.insert(i, n2);
                            i += 1;
                            change = true;
                        }
                    }
                }
                i += 1;
            }
        }

        result2 += l[l.len() / 2];
    }


    println!("{}", result2);
}

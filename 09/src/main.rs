use aoclib::read_input;

fn main() {
    let input = read_input("input.txt");

    let chars = input.trim_last_newline().chars().collect::<Vec<char>>();

    let mut i = 0;
    let mut pos = 0usize;
    let mut right_i = if chars.len() % 2 == 0 {
        chars.len() - 2
    } else {
        chars.len() - 1
    };
    let mut right_used = 0;

    let mut result = 0;

    while i < right_i {
        let num = chars[i].to_digit(10).unwrap() as usize;
        for j in pos..pos+num {
            result += (i / 2) * j;
        }
        pos += num;
        i += 1;

        let num = chars[i].to_digit(10).unwrap() as usize;
        let mut num_used = 0;
        let mut current_right = chars[right_i].to_digit(10).unwrap() as usize;
        while num_used < num && right_i > i {
            if right_used == current_right {
                right_i -= 2;
                right_used = 0;
                current_right = chars[right_i].to_digit(10).unwrap() as usize;
            } else {
                result += (right_i / 2) * (pos + num_used);
                right_used += 1;
                num_used += 1;
            }
        }
        pos += num;
        i += 1;
        // Take care of the rest if we reach the middle 
    }


    let rest = if right_i == i {
        chars[right_i].to_digit(10).unwrap() as usize - right_used
    } else {
        0
    };

    for j in pos..pos+rest {
        result += (i / 2) * j;
    }

    println!("Result: {}", result);

    // Part 2

    let mut free_list = vec![];
    let mut pos_index = vec![0; chars.len()];

    let mut pos = 0;

    for i in 0..chars.len() {
        pos_index[i] = pos;
        let num = chars[i].to_digit(10).unwrap() as usize;
        if i % 2 == 1 {
            free_list.push((pos, num));
        }
        pos += num;
    }

    let mut unmoved_list = chars.iter().enumerate().step_by(2).map(|(i, _)| i).collect::<Vec<usize>>();

    let mut result = 0;

    while let Some(elem) = unmoved_list.pop() {
        let num = chars[elem].to_digit(10).unwrap() as usize;
        let mut pos = None;
        for i in 0..free_list.len() {
            let potential_pos = &mut free_list[i];
            if potential_pos.0 > pos_index[elem] {
                break;
            }
            if potential_pos.1 >= num {
                pos = Some(potential_pos.0);
                if potential_pos.1 == num {
                    free_list.remove(i);
                } else {
                    free_list[i] = (potential_pos.0 + num, potential_pos.1 - num);
                }
                break;
            }
        }
        if let Some(pos) = pos {
            for i in pos..pos+num {
                result += (elem / 2) * i;
            }
            let prev_pos = pos_index[elem];
            free_list.insert(0, (prev_pos, num));
            free_list.sort();
        } else {
            let pos = pos_index[elem];
            for i in pos..pos+num {
                result += (elem / 2) * i;
            }
        }
    }

    println!("Result 2: {}", result);
}

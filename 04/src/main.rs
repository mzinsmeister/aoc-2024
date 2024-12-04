use std::collections::HashSet;

use aoclib::read_input;

fn search_word(word: &[char], input: &[Vec<char>], first_char: (usize, usize), delta: (isize, isize)) -> usize {
    let mut current_char = first_char;
    let mut matched_chars = 0;
    let mut matches = 0;
    loop {
        if input[current_char.0][current_char.1] == word[matched_chars] {
            matched_chars += 1;
            if matched_chars == word.len() {
                matches += 1;
                matched_chars = 0;
            }
        } else if input[current_char.0][current_char.1] == word[0] {
            matched_chars = 1;
        } else {
            matched_chars = 0;
        }

        let next_char = (current_char.0 as isize + delta.0, current_char.1 as isize + delta.1);
        if next_char.0 < 0 || next_char.0 >= input.len() as isize || next_char.1 < 0 || next_char.1 >= input[0].len() as isize {
            break;
        }
        current_char = (next_char.0 as usize, next_char.1 as usize);
    }
    matches
}

fn find_possible_middles(word: &[char], input: &[Vec<char>], first_char: (usize, usize), delta: (isize, isize)) -> Vec<(usize, usize)> {
    let mut current_char = first_char;
    let mut matched_chars = 0;
    let mut middles = vec![];
    loop {
        if input[current_char.0][current_char.1] == word[matched_chars] {
            matched_chars += 1;
            if matched_chars == word.len() {
                middles.push(((current_char.0 as isize - (word.len() as isize / 2 * delta.0)) as usize, (current_char.1 as isize - (word.len() as isize / 2 * delta.1)) as usize));
                matched_chars = 0;
            }
        } else if input[current_char.0][current_char.1] == word[0] {
            matched_chars = 1;
        } else {
            matched_chars = 0;
        }

        let next_char = (current_char.0 as isize + delta.0, current_char.1 as isize + delta.1);
        if next_char.0 < 0 || next_char.0 >= input.len() as isize || next_char.1 < 0 || next_char.1 >= input[0].len() as isize {
            break;
        }
        current_char = (next_char.0 as usize, next_char.1 as usize);
    }
    middles
}


fn main() {
    let input_raw = read_input("input.txt");

    let input: Vec<Vec<char>> = input_raw.trim_last_newline().lines().map(|line| line.chars().collect()).collect();

    let word = ['X', 'M', 'A', 'S'];

    let mut result = 0;

    // Line
    for i in 0..input.len() {
        let matches = search_word(&word, &input, (i, 0), (0, 1));
        result += matches;
    }

    // Reverse Line
    for i in 0..input.len() {
        let matches = search_word(&word, &input, (i, input[0].len() - 1), (0, -1));
        result += matches;
    }

    // Column
    for i in 0..input[0].len() {
        let matches = search_word(&word, &input, (0, i), (1, 0));
        result += matches;
    }

    // Reverse Column
    for i in 0..input[0].len() {
        let matches = search_word(&word, &input, (input.len() - 1, i), (-1, 0));
        result += matches;
    }

    // Diagonal top left to bottom right
    for i in 0..input.len() {
        let matches = search_word(&word, &input, (i, 0), (1, 1));
        result += matches;
    }
    for i in 1..input[0].len()-1 {
        let matches = search_word(&word, &input, (0, i), (1, 1));
        result += matches;
    }

    // Diagonal bottom right to top left
    for i in 0..input.len() {
        let matches = search_word(&word, &input, (i, input[0].len() - 1), (-1, -1));
        result += matches;
    }
    for i in 1..input[0].len()-1 {
        let matches = search_word(&word, &input, (input.len() - 1, i), (-1, -1));
        result += matches;
    }

    // Diagonal bottom left to top right
    for i in 0..input.len() {
        let matches = search_word(&word, &input, (i, 0), (-1, 1));
        result += matches;
    }
    for i in 1..input[0].len()-1 {
        let matches = search_word(&word, &input, (input.len() - 1, i), (-1, 1));
        result += matches;
    }

    // Diagonal top right to bottom left
    for i in 0..input.len() {
        let matches = search_word(&word, &input, (i, input[0].len() - 1), (1, -1));
        result += matches;
    }
    for i in 1..input[0].len()-1 {
        let matches = search_word(&word, &input, (0, i), (1, -1));
        result += matches;
    }



    println!("{}", result);
    
    let word = ['M', 'A', 'S'];

    let mut possible_middles: HashSet<(usize, usize)> = HashSet::new();


    // Diagonal top left to bottom right
    for i in 0..input.len() {
        let matches = find_possible_middles(&word, &input, (i, 0), (1, 1));
        possible_middles.extend(matches.iter());
    }
    for i in 1..input[0].len()-1 {
        let matches = find_possible_middles(&word, &input, (0, i), (1, 1));
        possible_middles.extend(matches.iter());
    }

    // Diagonal bottom right to top left
    for i in 0..input.len() {
        let matches = find_possible_middles(&word, &input, (i, input[0].len() - 1), (-1, -1));
        possible_middles.extend(matches.iter());
    }
    for i in 1..input[0].len()-1 {
        let matches = find_possible_middles(&word, &input, (input.len() - 1, i), (-1, -1));
        possible_middles.extend(matches.iter());
    }

    let mut result2 = 0;

    // Diagonal bottom left to top right
    for i in 0..input.len() {
        let matches = find_possible_middles(&word, &input, (i, 0), (-1, 1));
        for m in matches {
            if possible_middles.contains(&m) {
                result2 += 1;
            }
        }
    }
    for i in 1..input[0].len()-1 {
        let matches = find_possible_middles(&word, &input, (input.len() - 1, i), (-1, 1));
        for m in matches {
            if possible_middles.contains(&m) {
                result2 += 1;
            }
        }
    }

    // Diagonal top right to bottom left
    for i in 0..input.len() {
        let matches = find_possible_middles(&word, &input, (i, input[0].len() - 1), (1, -1));
        for m in matches {
            if possible_middles.contains(&m) {
                result2 += 1;
            }
        }
    }
    for i in 1..input[0].len()-1 {
        let matches = find_possible_middles(&word, &input, (0, i), (1, -1));
        for m in matches {
            if possible_middles.contains(&m) {
                result2 += 1;
            }
        }
    }

    println!("{}", result2);
}

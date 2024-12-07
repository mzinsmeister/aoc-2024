use aoclib::read_input;


// It's not smart but it's fast enough to run in under 10s even without --release

fn check_equation(result: usize, operands: &[usize]) -> bool {

    if operands.len() == 1 {
        return operands[0] == result;
    }

    let last = *operands.last().unwrap();

    // Check addition first
    if result > last && check_equation(result - last, &operands[0..operands.len() - 1]) {
        return true;
    }

    // Check multiplication
    if result % last == 0 && check_equation(result / last, &operands[0..operands.len() - 1]) {
        return true;
    }

    false
}

fn check_equation2(result: usize, operands: &[usize]) -> bool {

    if operands.len() == 1 {
        return operands[0] == result;
    }

    let last = *operands.last().unwrap();

    // Check addition first
    if result > last && check_equation2(result - last, &operands[0..operands.len() - 1]) {
        return true;
    }

    // Check multiplication
    if result % last == 0 && check_equation2(result / last, &operands[0..operands.len() - 1]) {
        return true;
    }

    // Check concatenation
    let last_string = last.to_string();
    let num_digits_base_10 = last_string.len() as u32; // This could be optimized
    if result.to_string().ends_with(&last_string) && check_equation2(result / 10usize.pow(num_digits_base_10), &operands[0..operands.len() - 1]) {
        return true;
    }

    false
}

fn main() {
    let input = read_input("input.txt");

    let parse = |v: &str| v.parse::<usize>().unwrap();

    let input = input.into_mapping(parse, parse, ": ", " ");

    let result = input.iter()
        .filter(|(result, operands)| check_equation(*result, operands))
        .map(|(result, _)| *result)
        .sum::<usize>();

    println!("Result: {}", result);

    let result2 = input.iter()
        .filter(|(result, operands)| check_equation2(*result, operands))
        .map(|(result, _)| *result)
        .sum::<usize>();

    println!("Result2: {}", result2);
}

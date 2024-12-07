use aoclib::read_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply
}

impl Operator {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator2 {
    Add,
    Multiply,
    Concat
}

impl Operator2 {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operator2::Add => a + b,
            Operator2::Multiply => a * b,
            Operator2::Concat => format!("{}{}", a, b).parse::<usize>().unwrap()
        }
    }
}

// It's not smart but it's fast enough to run in under 10s even without --release

fn check_equation(result: usize, operands: &[usize]) -> bool {

    let mut operators =  vec![Operator::Add; operands.len() - 1];

    for i in 0..(1 << operators.len()) {
        for j in 0..operators.len() {
            if i & (1 << j) != 0 {
                operators[j] = Operator::Multiply;
            } else {
                operators[j] = Operator::Add;
            }
        }

        let mut current_result = operands[0];
        for j in 0..operators.len() {
            let operator = operators[j];
            let operand = operands[j + 1];
            current_result = operator.apply(current_result, operand);
        }

        if current_result == result {
            //println!("{} = {}", operands.iter().skip(1).zip(operators.iter()).fold(operands[0].to_string(), |acc, (operand, operator)| format!("{} {} {}", acc, match operator { Operator::Add => "+", Operator::Multiply => "*" }, operand)), result);
            return true;
        }
    }

    false
}

fn check_equation2(result: usize, operands: &[usize]) -> bool {

    let mut operators =  vec![Operator2::Add; operands.len() - 1];

    for i in 0..(3usize.pow(operators.len() as u32)) {
        for j in 0..operators.len() {
            let op = (i / 3usize.pow(j as u32)) % 3;
            operators[j] = match op {
                0 => Operator2::Add,
                1 => Operator2::Multiply,
                2 => Operator2::Concat,
                _ => unreachable!()
            };
        }

        let mut current_result = operands[0];
        for j in 0..operators.len() {
            let operator = operators[j];
            let operand = operands[j + 1];
            current_result = operator.apply(current_result, operand);
        }

        if current_result == result {
            //println!("{} = {}", operands.iter().skip(1).zip(operators.iter()).fold(operands[0].to_string(), |acc, (operand, operator)| format!("{} {} {}", acc, match operator { Operator2::Add => "+", Operator2::Multiply => "*", Operator2::Concat => "||" }, operand)), result);
            return true;
        }
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

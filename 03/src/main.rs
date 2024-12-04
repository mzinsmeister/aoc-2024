

fn main() {
    let raw_input = aoclib::read_input("input.txt");

    let regex = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let result = regex.captures_iter(&raw_input).into_iter().map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap()).sum::<i32>();

    println!("Result1: {}", result);

    let regex = regex::Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\))").unwrap();

    let mut state = true;
    let mut result = 0;

    regex.captures_iter(&raw_input).into_iter().for_each(|c| {
        match &c[1] {
            "do()" => state = true,
            "don't()" => state = false,
            _ => if state { result += c[2].parse::<i32>().unwrap() * c[3].parse::<i32>().unwrap() }
        }
    });

    println!("Result2: {}", result);
}

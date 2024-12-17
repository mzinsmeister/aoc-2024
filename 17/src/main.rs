use std::ops::{BitOrAssign, BitXor};

use aoclib::read_input;


struct ComputerState {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    ip: usize
}

impl ComputerState {
    fn new(a: u64, b: u64, c: u64, program: Vec<u8>) -> ComputerState {
        ComputerState {
            a,
            b,
            c,
            program,
            ip: 0
        }
    }

    fn get_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0 | 1 | 2 | 3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand")
        }
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            match opcode {
                0 => {
                    self.a = self.a / (1 << self.get_combo_operand(operand));
                }
                1 => {
                    self.b = self.b.bitxor(operand as u64);
                }
                2 => {
                    self.b = self.get_combo_operand(operand) % 8;
                }
                3 => {
                    if self.a != 0 {
                        self.ip = operand as usize;
                        continue;
                    }
                }
                4 => {
                    self.b = self.b.bitxor(self.c);
                }
                5 => {
                    output.push((self.get_combo_operand(operand) % 8) as u8);
                }
                6 => {
                    self.b = self.a / (1 << self.get_combo_operand(operand));
                }
                7 => {
                    self.c = self.a / (1 << self.get_combo_operand(operand));
                }
                _ => panic!("Unknown opcode")
            }
            self.ip += 2;
        }
        output
    }

}

/*
This is what our program effectively does:

while a != 0 {
    b = a % 8 // ((a % 8) xor 2) xor ((a % 8) / (1 << ((a % 8) xor 2))) = 1
    b = b xor 2 // (b xor 2) xor (a / (1 << (b xor 2))) = 1
    c = a / (1 << b) // b xor (a / (1 << b)) = 1
    b = b xor c // b xor c = 1
    b = b xor 3 // output xor 3
    output b // 2
    a = a / 3
}

*/

fn solve2(cur: u64, rest: &[u8]) -> Option<u64> {
    if rest.len() == 0 {
        return Some(cur);
    }
    for i in 0..8 {
        let a = cur * 8 + i;
        let mut b = a % 8;
        b = b.bitxor(2);
        let c = a / (1 << b);
        b = b.bitxor(c);
        b = b.bitxor(3);
        if b % 8 == rest[rest.len() - 1] as u64 {
            let val = try_value(a, &rest[..rest.len() - 1]);
            if let Some(val) = val {
                return Some(val);
            }
        }
    }
    None
}

fn main() {
    let input = read_input("input.txt");

    let (regs_raw, program_raw) = input.split_once("\n\n");

    let mut regs_iter = regs_raw.split("\n").map(|line| {
        line[12..line.len()].parse::<u64>().unwrap()
    });

    let a = regs_iter.next().unwrap();
    let b = regs_iter.next().unwrap();
    let c = regs_iter.next().unwrap();

    let program = program_raw.trim_last_newline().trim_start_matches("Program: ").split(",").map(|n| {
        n.parse::<u8>().unwrap()
    }).collect::<Vec<u8>>();

    let mut computer = ComputerState::new(a, b, c, program.clone());

    let result = computer.run().iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");

    println!("Part 1: {}", result);

    // Part 2

    let val = solve2(0, &program).unwrap();


    println!("Part 2: {}", val);

}

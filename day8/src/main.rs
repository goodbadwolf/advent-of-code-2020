use std::{collections::HashSet, fs};

#[derive(Clone)]
pub enum Inst {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

pub enum Status {
    Infinite,
    Correct,
}

fn parse_insts(input: &String) -> Vec<Inst> {
    input
        .split("\n")
        .map(|inst| {
            let (op, val) = inst.split_at(3);
            let (sign, val) = (
                val.chars().next().unwrap(),
                val[1..].parse::<i32>().unwrap(),
            );
            let val = if sign == '-' { -val } else { val };
            if op == "acc" {
                Inst::Acc(val)
            } else if op == "jmp" {
                Inst::Jmp(val)
            } else {
                Inst::Nop(val)
            }
        })
        .collect()
}

fn execute_code(code: &Vec<Inst>) -> (Status, i32) {
    let mut acc = 0;
    let mut pc: usize = 0;
    let mut seen_code: HashSet<usize> = HashSet::new();
    let status = loop {
        if seen_code.contains(&pc) {
            break Status::Infinite;
        } else if pc == code.len() {
            break Status::Correct;
        }
        seen_code.insert(pc);
        let inst = &code[pc];
        match inst {
            Inst::Acc(delta) => {
                acc += *delta;
                pc += 1;
            }
            Inst::Jmp(delta) => {
                pc = (pc as i32 + delta) as usize;
            }
            Inst::Nop(_) => {
                pc += 1;
            }
        }
    };

    (status, acc)
}

fn fix_code(boot_code: &Vec<Inst>) -> i32 {
    for (i, inst) in boot_code.iter().enumerate() {
        let mut fixed_code = boot_code.clone();
        let fixed_inst = match inst {
            Inst::Nop(val) => Inst::Jmp(*val),

            Inst::Jmp(val) => Inst::Nop(*val),

            Inst::Acc(val) => Inst::Acc(*val),
        };
        fixed_code[i] = fixed_inst;
        match execute_code(&fixed_code) {
            (Status::Infinite, _) => {
                continue;
            }
            (Status::Correct, acc) => {
                return acc;
            }
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let boot_code = parse_insts(&input);
    let (_, acc) = execute_code(&boot_code);
    println!("Acc: {}", acc);

    let acc = fix_code(&boot_code);
    println!("Fixed acc: {}", acc);
}

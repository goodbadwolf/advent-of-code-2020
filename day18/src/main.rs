use std::fs;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Num(i64),
    OpenBracket,
    CloseBracket,
    Operator(char),
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn calculate_expression(exp: &str, use_precedence: bool) -> i64 {
    let tokens_re = Regex::new(r"([\d\+\*]+|[\(\)])").unwrap();
    let tokens = tokens_re.captures_iter(exp);
    let mut val_stack = vec![];
    let mut op_stack = vec![];
    for token in tokens {
        let token = &token[0];
        let is_num = token.chars().all(char::is_numeric);
        if token.eq("(") {
            op_stack.push('(');
        } else if is_num {
            let num = token.parse::<i64>().unwrap();
            val_stack.push(num);
        } else if token.eq(")") {
            while !op_stack.is_empty() && *(op_stack.last().unwrap()) != '(' {
                let rhs = val_stack.pop().unwrap();
                let lhs = val_stack.pop().unwrap();
                let op = op_stack.pop().unwrap();
                let result = match op {
                    '+' => lhs + rhs,
                    '*' => lhs * rhs,
                    _ => {
                        panic!("Unknown operator: {}", op);
                    }
                };
                val_stack.push(result);
            }
            if !op_stack.is_empty() {
                op_stack.pop().unwrap();
            }
        } else {
            while !op_stack.is_empty()
                && precedence(*(op_stack.last().unwrap()), use_precedence)
                    >= precedence(token.chars().next().unwrap(), use_precedence)
            {
                let rhs = val_stack.pop().unwrap();
                let lhs = val_stack.pop().unwrap();
                let op = op_stack.pop().unwrap();
                let result = match op {
                    '+' => lhs + rhs,
                    '*' => lhs * rhs,
                    _ => {
                        panic!("Unknown operator: {}", op);
                    }
                };
                val_stack.push(result);
            }
            op_stack.push(token.chars().next().unwrap());
        }
    }
    while !op_stack.is_empty() {
        let rhs = val_stack.pop().unwrap();
        let lhs = val_stack.pop().unwrap();
        let op = op_stack.pop().unwrap();
        let result = match op {
            '+' => lhs + rhs,
            '*' => lhs * rhs,
            _ => {
                panic!("Unknown operator: {}", op);
            }
        };

        val_stack.push(result);
    }

    *(val_stack.last().unwrap())
}

fn precedence(op: char, use_precedence: bool) -> i32 {
    match op {
        '+' => {
            if use_precedence {
                2
            } else {
                1
            }
        }
        '*' => 1,
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let exprs = parse_input(&input);
    let part1_result: i64 = exprs
        .iter()
        .map(|exp| calculate_expression(exp, false))
        .sum();
    println!("Part 1: {}", part1_result);
    let part2_result: i64 = exprs
        .iter()
        .map(|exp| calculate_expression(exp, true))
        .sum();
    println!("Part 2: {}", part2_result);
}

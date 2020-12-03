#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug, Clone)]
pub struct Input {
    pub low: u32,
    pub high: u32,
    pub letter: char,
    pub password: String,
}

fn parse_input_line(line: &str) -> Input {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s([a-z]{1}):\s(\w*)$").unwrap();
    }

    let captures = RE.captures(line).unwrap();
    Input {
        low: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        high: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        letter: captures.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        password: captures.get(4).unwrap().as_str().to_string(),
    }
}

fn parse_input(input: &str) -> Vec<Input> {
    input.lines().map(|line| parse_input_line(line)).collect()
}

fn retain_sled_valid_passwords(inputs: Vec<Input>) -> Vec<Input> {
    inputs
        .into_iter()
        .filter(|i| {
            let letter_count = i.password.chars().filter(|&c| c == i.letter).count() as u32;
            letter_count >= i.low && letter_count <= i.high
        })
        .collect()
}

fn retain_toboggan_valid_passwords(inputs: Vec<Input>) -> Vec<Input> {
    inputs
        .into_iter()
        .filter(|i| {
            let password: Vec<char> = i.password.chars().collect();
            let low_char = password[(i.low - 1) as usize];
            let high_char = password[(i.high - 1) as usize];
            (low_char == i.letter) ^ (high_char == i.letter)
        })
        .collect()
}

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let inputs = parse_input(&inputs);
    let sled_inputs = inputs.clone();
    let toboggan_inputs = inputs.clone();

    let valid_passwords = retain_sled_valid_passwords(sled_inputs);
    println!("Valid sled passwords count: {}", valid_passwords.len());
    let valid_passwords = retain_toboggan_valid_passwords(toboggan_inputs);
    println!("Valid tobogan passwords count: {}", valid_passwords.len());
}

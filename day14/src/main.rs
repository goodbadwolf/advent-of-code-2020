#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs};

pub enum Inst {
    Mask(String),
    Mem(u64, u64),
}

fn parse_input(input: &str) -> Vec<Inst> {
    lazy_static! {
        static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\]\s=\s(\d+)$").unwrap();
    }

    let input = input.lines();
    return input
        .map(|inst| {
            if inst[0..4].to_string() == "mask" {
                Inst::Mask(inst[7..].to_string())
            } else {
                let captures = MEM_RE.captures(inst).unwrap();
                let mem = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let val = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                Inst::Mem(mem, val)
            }
        })
        .collect();
}

fn execute_program_v1(program: &Vec<Inst>, memory: &mut HashMap<u64, u64>) {
    let mut mask = vec!['X'; 36];
    for inst in program {
        match inst {
            Inst::Mask(new_mask) => mask = new_mask.chars().rev().collect(),

            Inst::Mem(mem, val) => {
                let mut masked_val = *val;
                for i in 0..36_u64 {
                    let mask_bit = mask[i as usize];
                    if mask_bit == '0' {
                        masked_val &= !(1 << i);
                    } else if mask_bit == '1' {
                        masked_val |= 1 << i;
                    }
                }
                memory.insert(*mem, masked_val);
            }
        }
    }
}

fn to_bits(val: u64) -> Vec<char> {
    let mut bits = vec!['0'; 36];
    for i in 0..36 {
        bits[i] = if (val >> i & 1) == 1 { '1' } else { '0' };
    }

    bits
}

fn execute_program_v2(program: &Vec<Inst>, memory: &mut HashMap<u64, u64>) {
    let mut mask = vec!['0'; 36];
    for inst in program {
        match inst {
            Inst::Mask(new_mask) => mask = new_mask.chars().rev().collect(),

            Inst::Mem(mem, val) => {
                let mut floating_mask = vec!['0'; 36];
                let mem = to_bits(*mem);
                for i in 0..36_u64 {
                    let idx = i as usize;
                    let mask_bit = mask[idx];
                    if mask_bit == '0' {
                        floating_mask[idx] = mem[idx];
                    } else {
                        floating_mask[idx] = mask_bit;
                    }
                }
                let num_floating = floating_mask.iter().filter(|&&c| c == 'X').count();
                for combination in 0..(1 << num_floating) {
                    let mut mem = 0_u64;
                    let mut floating_idx = num_floating;
                    for j in 0..36 {
                        let mask_bit = if floating_mask[j] == 'X' {
                            let mask_bit = if (combination >> (floating_idx - 1)) & 1 == 1 {
                                '1'
                            } else {
                                '0'
                            };
                            floating_idx -= 1;
                            mask_bit
                        } else {
                            floating_mask[j]
                        };
                        let mask_bit = if mask_bit == '1' { 1 } else { 0 };
                        mem += mask_bit * 2_u64.pow((35 - j) as u32);
                    }
                    memory.insert(mem, *val);
                }
            }
        }
    }
}

fn calculate_memory_sum(memory: &HashMap<u64, u64>) -> u64 {
    let mut sum = 0;
    for (_, val) in memory {
        sum += *val;
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let program = parse_input(&input);

    let mut v1_memory: HashMap<u64, u64> = HashMap::new();
    execute_program_v1(&program, &mut v1_memory);
    println!("V1 Decoder: {}", calculate_memory_sum(&v1_memory));

    let mut v2_memory: HashMap<u64, u64> = HashMap::new();
    execute_program_v2(&program, &mut v2_memory);
    println!("V2 Decoder: {}", calculate_memory_sum(&v2_memory));
}

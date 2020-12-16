#![feature(str_split_once)]
use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub ranges: Vec<(u32, u32)>,
}

#[derive(Debug)]
pub struct Input {
    pub rules: Vec<Rule>,
    pub your_ticket: Vec<u32>,
    pub nearby_tickets: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();
    let your_ticket = parts.next().unwrap().lines().nth(1).unwrap();
    let nearby_tickets_str = parts.next().unwrap();
    let mut nearby_tickets = vec![];
    for ticket in nearby_tickets_str.lines().skip(1) {
        nearby_tickets.push(parse_ticket(ticket));
    }

    Input {
        rules: parse_rules(&rules),
        your_ticket: parse_ticket(your_ticket),
        nearby_tickets,
    }
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules
        .lines()
        .map(|line| {
            let (name, ranges) = line.split_once(": ").unwrap();
            let (first, second) = ranges.split_once(" or ").unwrap();
            let mut ranges = vec![];
            for range in &[first, second] {
                let (min, max) = range.split_once('-').unwrap();
                ranges.push((min.parse::<u32>().unwrap(), max.parse::<u32>().unwrap()));
            }
            Rule {
                name: name.to_string(),
                ranges,
            }
        })
        .collect()
}

fn parse_ticket(ticket_line: &str) -> Vec<u32> {
    ticket_line
        .split(',')
        .map(|val| val.parse::<u32>().unwrap())
        .collect()
}

fn is_valid_value(rules: &[Rule], val: u32) -> bool {
    for rule in rules {
        for &(min, max) in rule.ranges.iter() {
            if val >= min && val <= max {
                return true;
            }
        }
    }

    false
}

fn find_invalid_values(input: &Input) -> Vec<u32> {
    let mut invalids = vec![];
    for ticket in input.nearby_tickets.iter() {
        for val in ticket {
            if !is_valid_value(&input.rules, *val) {
                invalids.push(*val);
            }
        }
    }

    invalids
}

fn find_field_names(input: &Input) -> HashMap<String, usize> {
    let mut valid_tickets = input.nearby_tickets.clone();
    valid_tickets.retain(|ticket| {
        for val in ticket {
            if !is_valid_value(&input.rules, *val) {
                return false;
            }
        }

        true
    });

    let mut fields_map: HashMap<String, usize> = HashMap::new();
    let mut mapped_cols = vec![];
    while mapped_cols.len() < input.rules.len() {
        for rule in input.rules.iter() {
            let mut possible_cols = vec![];
            for col in 0..input.rules.len() {
                if mapped_cols.contains(&col) {
                    continue;
                }
                let mut satisfies: bool = true;
                for ticket in valid_tickets.iter() {
                    let val = ticket[col];
                    let mut range_satisfies = false;
                    for (min, max) in rule.ranges.iter() {
                        if val >= *min && val <= *max {
                            range_satisfies = true;
                        }
                    }
                    if !range_satisfies {
                        satisfies = false;
                    }
                }
                if satisfies {
                    possible_cols.push(col);
                }
            }
            if possible_cols.len() == 1 {
                let col = possible_cols.first().unwrap();
                fields_map.insert(rule.name.clone(), *col);
                mapped_cols.push(*col);
            }
        }
    }

    fields_map
}

fn multiply_fields(ticket: &[u32], fields_map: &HashMap<String, usize>, fields: &[String]) -> u64 {
    fields.iter().fold(1_u64, |acc, field| {
        let idx = fields_map[field];
        acc * ticket[idx] as u64
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let input = parse_input(&input);

    let invalid_values = find_invalid_values(&input);
    let error_rate: u32 = invalid_values.iter().sum();
    println!("Scanning error rate: {}", error_rate);

    let fields_map = find_field_names(&input);
    let result = multiply_fields(
        &input.your_ticket,
        &fields_map,
        &[
            "departure location".to_string(),
            "departure station".to_string(),
            "departure platform".to_string(),
            "departure track".to_string(),
            "departure date".to_string(),
            "departure time".to_string(),
        ],
    );
    println!("Product of departure fields: {}", result);
}

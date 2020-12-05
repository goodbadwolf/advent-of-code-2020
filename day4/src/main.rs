#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::fs;

fn parse_input(input: &String) -> Vec<String> {
    let passports = input.split("\n\n");
    passports.map(|p| p.to_string()).collect()
}

fn parse_passport_fields(passport: &String) -> Vec<(String, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(([a-z]{3}):([a-zA-Z0-9#]+)\s*)").unwrap();
    }

    RE.captures_iter(passport)
        .map(|capture| (capture[2].to_string(), capture[3].to_string()))
        .collect()
}

fn find_valid_part1_passports(passports: Vec<String>, req_keys: Vec<&str>) -> Vec<String> {
    passports
        .into_iter()
        .filter(|passport| {
            let keys = parse_passport_fields(&passport)
                .iter()
                .map(|(key, _)| key.clone())
                .collect::<Vec<String>>();
            let unmatched_keys = req_keys
                .iter()
                .filter(|&&key| !keys.contains(&key.to_string()));
            unmatched_keys.count() == 0
        })
        .collect()
}

fn validate_fields(fields: &Vec<(String, String)>) -> bool {
    let mut result = true;

    for (key, value) in fields {
        if key.eq("byr") {
            let byr = value.parse::<u32>().unwrap_or(0);
            result = result && (byr >= 1920 && byr <= 2002);
        } else if key.eq("iyr") {
            let iyr = value.parse::<u32>().unwrap_or(0);
            result = result && (iyr >= 2010 && iyr <= 2020);
        } else if key.eq("eyr") {
            let eyr = value.parse::<u32>().unwrap_or(0);
            result = result && (eyr >= 2020 && eyr <= 2030);
        } else if key.eq("hgt") {
            lazy_static! {
                static ref HGT_RE: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
            }
            if let Some(height_cap) = HGT_RE.captures(value) {
                let t = height_cap.get(2).unwrap().as_str();
                let height = height_cap
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap_or(0);
                if t.eq("cm") {
                    result = result && (height >= 150 && height <= 193);
                } else if t.eq("in") {
                    result = result && (height >= 59 && height <= 76);
                } else {
                    result = false;
                }
            } else {
                result = false;
            }
        } else if key.eq("hcl") {
            lazy_static! {
                static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }
            result = result && HCL_RE.is_match(value);
        } else if key.eq("ecl") {
            result = result
                && vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str());
        } else if key.eq("pid") {
            lazy_static! {
                static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
            }
            result = result && PID_RE.is_match(value);
        }
        if !result {
            return false;
        }
    }

    result
}

fn find_valid_part2_passports(passports: Vec<String>, req_keys: Vec<&str>) -> Vec<String> {
    passports
        .into_iter()
        .filter(|passport| {
            let fields = parse_passport_fields(&passport);
            let keys = fields
                .iter()
                .map(|(key, _)| key.clone())
                .collect::<Vec<String>>();
            let unmatched_keys = req_keys
                .iter()
                .filter(|&&key| !keys.contains(&key.to_string()));
            (unmatched_keys.count() == 0) && validate_fields(&fields)
        })
        .collect()
}

const REQ_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let passports = parse_input(&inputs);
    let valid_passports = find_valid_part1_passports(passports.clone(), REQ_KEYS.to_vec());
    println!("Part1 valid passports count: {}", valid_passports.len());
    let valid_passports = find_valid_part2_passports(passports, REQ_KEYS.to_vec());
    println!("Part2 valid passports count: {}", valid_passports.len());
}

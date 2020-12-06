use std::collections::HashSet;
use std::fs;

fn parse_input(input: &String) -> Vec<String> {
    input.split("\n\n").map(|p| p.to_string()).collect()
}

fn calculate_unique_answer_counts(group_answers: &Vec<String>) -> Vec<u32> {
    group_answers
        .into_iter()
        .map(|answers| {
            let chars: HashSet<char> = answers
                .chars()
                .filter(|c| !c.is_whitespace() && c.is_alphabetic())
                .collect();
            chars.len() as u32
        })
        .collect()
}

fn calculate_common_answer_counts(group_answers: &Vec<String>) -> Vec<u32> {
    group_answers
        .into_iter()
        .map(|answers| {
            let individual_answers: Vec<HashSet<char>> = answers
                .split_ascii_whitespace()
                .map(|answer| answer.chars().collect())
                .collect();
            
            let common_chars = individual_answers[0].iter().filter(|&c|
                individual_answers.iter().all(|a| a.contains(c))
            );

            common_chars.count() as u32
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let group_answers = parse_input(&input);
    let group_unique_answers_counts = calculate_unique_answer_counts(&group_answers);
    let group_common_answers_counts = calculate_common_answer_counts(&group_answers);
    let unique_counts_sum: u32 = group_unique_answers_counts.iter().sum();
    let common_counts_sum: u32 = group_common_answers_counts.iter().sum();
    println!("Sum of unique counts: {}", unique_counts_sum);
    println!("Sum of common counts: {}", common_counts_sum);
}

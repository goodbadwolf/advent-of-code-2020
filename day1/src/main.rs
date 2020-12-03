use std::fs;

fn parse_input(input: &String) -> Vec<i32> {
    input.lines().map(|i| i.parse::<i32>().unwrap()).collect()
}

fn solve(inputs: Vec<i32>) -> (i32, i32) {
    let lhs_result = *(inputs
        .iter()
        .find(|&&i| inputs.iter().position(|&j| j == 2020 - i).is_some())
        .unwrap());
    return (lhs_result, (2020 - lhs_result));
}

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Couldn't read input file");

    let inputs = parse_input(&inputs);

    let (lhs_expense, rhs_expense) = solve(inputs);

    println!(
        "{} x {} = {}",
        lhs_expense,
        rhs_expense,
        (lhs_expense * rhs_expense)
    );
}

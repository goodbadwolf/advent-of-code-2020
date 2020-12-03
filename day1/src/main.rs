use std::fs;

const EXPENSES_SUM: i32 = 2020;

fn parse_input(input: &String) -> Vec<i32> {
    input.lines().map(|i| i.parse::<i32>().unwrap()).collect()
}

fn find_two_expenses(inputs: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    let expenses = inputs
        .iter()
        .find(|&&i| inputs.iter().position(|&j| j == sum - i).is_some());
    match expenses {
        Some(&lhs_expense) => Some((lhs_expense, sum - lhs_expense)),
        None => None,
    }
}

fn find_three_expenses(inputs: &Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    inputs
        .iter()
        .find_map(|&i| match find_two_expenses(inputs, sum - i) {
            Some((middle_expense, right_expense)) => Some((i, middle_expense, right_expense)),
            None => None,
        })
}

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Couldn't read input file");

    let inputs = parse_input(&inputs);

    if let Some((lhs_expense, rhs_expense)) = find_two_expenses(&inputs, EXPENSES_SUM) {
        println!(
            "{} x {} = {}",
            lhs_expense,
            rhs_expense,
            (lhs_expense * rhs_expense)
        );
    }

    if let Some((l_expense, m_expense, r_expense)) = find_three_expenses(&inputs, EXPENSES_SUM) {
        println!(
            "{} x {} x {} = {}",
            l_expense,
            m_expense,
            r_expense,
            l_expense * m_expense * r_expense
        );
    }
}

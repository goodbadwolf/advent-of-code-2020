use std::collections::HashMap;

fn run_game(input: &Vec<u32>, last_turn: usize) -> u32 {
    let mut turns: HashMap<u32, usize> = HashMap::new();
    let mut turn = 1;
    while turn <= input.len() {
        turns.insert(input[turn - 1], turn);
        turn += 1;
    }

    let mut last = *input.last().unwrap();
    while turn <= last_turn {
        let this = if turns.contains_key(&last) {
            (turn - turns[&last] - 1) as u32
        } else {
            0_u32
        };
        turns.insert(last, turn - 1);
        last = this;
        turn += 1;
    }

    last
}

fn main() {
    let input = vec![14, 3, 1, 0, 9, 5];
    let result_1 = run_game(&input, 2020);
    println!("2020th = {}", result_1);
    let result_2 = run_game(&input, 30000000);
    println!("30000000th = {}", result_2);
}

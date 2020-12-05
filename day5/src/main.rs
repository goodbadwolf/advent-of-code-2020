use std::fs;

fn parse_input(input: &String) -> Vec<String> {
    input
        .split_ascii_whitespace()
        .map(|p| p.to_string())
        .collect()
}

fn calculate_seats(passes: Vec<String>) -> Vec<(u32, u32)> {
    passes
        .into_iter()
        .map(|pass| {
            let row_bsp = &pass[0..7];
            let col_bsp = &pass[7..];
            let row = calculate_dim(&row_bsp, 'F', 'B');
            let col = calculate_dim(&col_bsp, 'L', 'R');
            (row, col)
        })
        .collect()
}

fn calculate_dim(bsp: &str, low: char, high: char) -> u32 {
    let mut low_i = 0_u32;
    let mut high_i = 2_u32.pow(bsp.len() as u32) - 1;
    for s in bsp.chars() {
        if s == low {
            high_i = (low_i + high_i) / 2;
        } else if s == high {
            low_i = (low_i + high_i + 1) / 2;
        }
    }

    (low_i + high_i) / 2
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let passes = parse_input(&input);
    let seats = calculate_seats(passes);
    let mut seat_ids: Vec<u32> = seats.iter().map(|(r, c)| *r * 8_u32 + *c).collect();
    seat_ids.sort();
    
    let max_seat_id = seat_ids.last().unwrap();
    println!("Max seat ID: {}", max_seat_id);

    for i in 1..seat_ids.len() {
        if seat_ids[i] - seat_ids[i - 1] == 2 {
            println!("My seat: {}", seat_ids[i] - 1);
        }
    }
}

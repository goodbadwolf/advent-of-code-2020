use std::fs;

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let lines = input.lines();
    lines.map(|line| line.chars().collect()).collect()
}

fn find_tree_encounters(world: &Vec<Vec<char>>, slope: (usize, usize)) -> i32 {
    let (mut x, mut y) = (0_usize, 0_usize);
    let mut encounters = 0;
    let width = world[0].len();
    let height = world.len();
    while y < height - slope.1 {
        x = (x + slope.0) % width;
        y += slope.1;
        encounters += (world[y][x] == '#') as i32;
    }

    encounters
}

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let inputs = parse_input(&inputs);

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result: i64 = 1;
    for slope in slopes {
        let tree_encounters = find_tree_encounters(&inputs, slope);
        println!("{}, {}: {} tree encounters", slope.0, slope.1, tree_encounters);
        result *= tree_encounters as i64;
    }

    println!("Result: {}", result);
}

use std::fs;

#[derive(Copy, Clone)]
pub struct Rules {
    pub occupied_max: u32,
    pub adjacency_dist: i32,
}

fn parse_world(input: &String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|i| i.chars().collect::<Vec<char>>())
        .collect()
}

fn simulate_step(world: &Vec<Vec<char>>, rules: Rules) -> Vec<Vec<char>> {
    let mut next_world = world.clone();
    for i in 0..world.len() {
        for j in 0..world[i].len() {
            let c = world[i][j];
            if c == 'L' || c == '#' {
                let occupied = count_adjacent_occupied_seats(&world, i, j, rules);
                if c == 'L' && occupied == 0 {
                    next_world[i][j] = '#';
                } else if c == '#' && occupied >= rules.occupied_max {
                    next_world[i][j] = 'L';
                }
            } else {
                next_world[i][j] = c;
            }
        }
    }
    next_world
}

fn count_adjacent_occupied_seats(world: &Vec<Vec<char>>, i: usize, j: usize, rules: Rules) -> u32 {
    let mut occupied = 0;
    for ii in -1_i32..=1_i32 {
        for jj in -1_i32..=1_i32 {
            let mut mult = 1;
            while mult <= rules.adjacency_dist {
                if ii == 0 && jj == 0 {
                    break;
                }
                let i_o = i as i32 + mult * ii;
                let j_o = j as i32 + mult * jj;
                if i_o < 0 || i_o >= world.len() as i32 || j_o < 0 || j_o >= world[i].len() as i32 {
                    break;
                }

                let i_o = i_o as usize;
                let j_o = j_o as usize;
                if world[i_o][j_o] == '#' {
                    occupied += 1;
                    break;
                } else if world[i_o][j_o] == 'L' {
                    break;
                }

                mult += 1;
            }
        }
    }

    occupied
}

fn find_stable_occupied_seats(world: &Vec<Vec<char>>, rules: Rules) -> u64 {
    let mut curr_world = world.clone();
    let mut is_stable = false;
    while !is_stable {
        let next_world = simulate_step(&curr_world, rules);
        is_stable = true;
        for i in 0..world.len() {
            for j in 0..world[i].len() {
                if curr_world[i][j] != next_world[i][j] {
                    is_stable = false;
                }
            }
        }
        curr_world = next_world;
    }

    let occupied = curr_world
        .iter()
        .map(|r| r.iter().filter(|&&c| c == '#').count() as u64)
        .sum();
    return occupied;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let world = parse_world(&input);
    let stable_occupied_seats = find_stable_occupied_seats(
        &world,
        Rules {
            occupied_max: 4,
            adjacency_dist: 1,
        },
    );
    println!(
        "Stable occupied seats for part 1: {}",
        stable_occupied_seats
    );
    let stable_occupied_seats = find_stable_occupied_seats(
        &world,
        Rules {
            occupied_max: 5,
            adjacency_dist: world.len() as i32 + world[0].len() as i32,
        },
    );
    println!(
        "Stable occupied seats for part 2: {}",
        stable_occupied_seats
    );
}

use std::{collections::HashMap, fs};

type Coords = (i32, i32, i32, i32);

fn parse_input(input: &str) -> (HashMap<Coords, char>, i32, i32) {
    let mut state = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let y = y as i32;
        line.chars().enumerate().for_each(|(x, char)| {
            let x = x as i32;
            let z = 0;
            let w = 0;
            state.insert((x, y, z, w), char);
        })
    });

    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;

    (state, width, height)
}

fn read_state(state: &HashMap<Coords, char>, coords: &Coords) -> char {
    *state.get(coords).unwrap_or(&'.')
}

fn calculate_next_cell_state(
    cell: &(&Coords, &char),
    state: &HashMap<Coords, char>,
    use_4th_dim: bool,
) -> char {
    let coords = cell.0;
    let cell_state = read_state(state, coords);
    let mut active_neighbours_count = 0;
    let dw_range = if use_4th_dim { (-1, 1) } else { (0, 0) };
    for dw in dw_range.0..=dw_range.1 {
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let x = coords.0 + dx;
                    let y = coords.1 + dy;
                    let z = coords.2 + dz;
                    let w = coords.3 + dw;
                    if x == coords.0 && y == coords.1 && z == coords.2 && w == coords.3 {
                        continue;
                    }
                    let neighbour_state = read_state(state, &(x, y, z, w));
                    if neighbour_state == '#' {
                        active_neighbours_count += 1;
                    }
                }
            }
        }
    }

    if cell_state == '#' {
        if active_neighbours_count == 2 || active_neighbours_count == 3 {
            '#'
        } else {
            '.'
        }
    } else if active_neighbours_count == 3 {
        '#'
    } else {
        '.'
    }
}

fn simulate_cycles(
    state: &mut HashMap<Coords, char>,
    width: i32,
    height: i32,
    num_cycles: u32,
    use_4th_dim: bool,
) {
    let mut x_range = (0, width);
    let mut y_range = (0, height);
    let mut z_range = (0, 1);
    let mut w_range = (0, 1);
    for _cycle in 1..=num_cycles {
        x_range = (x_range.0 - 1, x_range.1 + 1);
        y_range = (y_range.0 - 1, y_range.1 + 1);
        z_range = (z_range.0 - 1, z_range.1 + 1);
        if use_4th_dim {
            w_range = (w_range.0 - 1, w_range.1 + 1);
        }
        let mut next_state = HashMap::new();
        for w in w_range.0..w_range.1 {
            for z in z_range.0..z_range.1 {
                for y in y_range.0..y_range.1 {
                    for x in x_range.0..x_range.1 {
                        let coords = &(x, y, z, w);
                        let cell_val = read_state(state, coords);
                        let next_cell_state =
                            calculate_next_cell_state(&(coords, &cell_val), state, use_4th_dim);
                        next_state.insert(*coords, next_cell_state);
                    }
                }
            }
        }
        *state = next_state;
    }
}

fn find_active_cubes(state: &HashMap<Coords, char>) -> u32 {
    let mut active = 0;
    for cell in state.values() {
        active += if *cell == '#' { 1 } else { 0 }
    }

    active
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let (initial_state, width, height) = parse_input(&input);

    let mut part1_state = initial_state.clone();
    simulate_cycles(&mut part1_state, width, height, 6, false);
    println!("3dims active cubes: {}", find_active_cubes(&part1_state));

    let mut part2_state = initial_state.clone();
    simulate_cycles(&mut part2_state, width, height, 6, true);
    println!("4dims active cubes: {}", find_active_cubes(&part2_state));
}

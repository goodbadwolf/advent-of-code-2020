use std::collections::HashMap;
use std::fs;

fn parse_adapters(input: &String) -> Vec<i32> {
    input
        .split("\n")
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn find_differences(adapters: &Vec<i32>) -> (i32, i32) {
    let mut diffs_1 = 0;
    let mut diffs_3 = 0;
    for i in 1..adapters.len() {
        match adapters[i] - adapters[i - 1] {
            1 => {
                diffs_1 += 1;
            }
            3 => {
                diffs_3 += 1;
            }
            _ => {}
        }
    }

    (diffs_1, diffs_3)
}

fn find_ways(
    adapters: &Vec<i32>,
    max_adapter_idx: usize,
    cache: &mut HashMap<usize, i128>,
) -> i128 {
    if cache.contains_key(&max_adapter_idx) {
        return cache[&max_adapter_idx];
    } else if max_adapter_idx == 0 {
        return 1;
    }

    let max_adapter = adapters[max_adapter_idx];
    let mut ways = 0;
    let min = (max_adapter_idx as i32 - 4).max(0) as usize;
    for i in min..max_adapter_idx {
        if max_adapter - adapters[i] <= 3 && max_adapter - adapters[i] > 0 {
            ways += find_ways(adapters, i, cache);
        }
    }
    cache.insert(max_adapter_idx, ways);

    ways
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let mut adapters = parse_adapters(&input);
    adapters.sort();
    adapters.insert(0, 0); // Outlet
    adapters.insert(adapters.len(), *adapters.last().unwrap() + 3); // Device
    let diffs = find_differences(&mut adapters);
    println!("Part 1: {}", diffs.0 * diffs.1);

    let total_ways = find_ways(&adapters, adapters.len() - 1, &mut HashMap::new());
    print!("Part 2: {}", total_ways);
}

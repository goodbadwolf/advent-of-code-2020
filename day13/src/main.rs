use std::fs;

fn parse_input(input: &str) -> (u64, Vec<(u64, u64)>) {
    let mut input = input.lines();
    let timestamp = input.next().unwrap().parse::<u64>().unwrap();
    let bus_schedule = input
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(idx, str)| match str.parse() {
            Ok(bus_id) => Some((idx as u64, bus_id)),
            _ => None,
        })
        .collect();

    (timestamp, bus_schedule)
}

fn find_closest_bus(timestamp: u64, bus_schedule: &[(u64, u64)]) -> (u64, u64) {
    let closest = bus_schedule
        .iter()
        .min_by_key(|&bus| (((timestamp / bus.1) + 1) * bus.1) - timestamp)
        .unwrap();
    let diff = (((timestamp / closest.1) + 1) * closest.1) - timestamp;
    (closest.1, diff)
}

fn find_timestamp_for_part2(bus_schedule: &[(u64, u64)]) -> u64 {
    let mut earliest_id = 1;
    let mut earliest_offset = 0;
    for bus in bus_schedule {
        let (bus_offset, bus_id) = *bus;
        for i in 0..=bus_id {
            let earliest_multiple = earliest_id * i;
            if (earliest_multiple + earliest_offset + bus_offset) % bus_id == 0 {
                earliest_id *= bus_id;
                earliest_offset += earliest_multiple;
                break;
            }
        }
    }

    earliest_offset
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let (timestamp, bus_schedule) = parse_input(&input);
    let (bus_id, closest) = find_closest_bus(timestamp, &bus_schedule);
    println!("Part 1: {}", bus_id * closest);
    let gold_star_timestamp = find_timestamp_for_part2(&bus_schedule);
    println!("Part 2: {}", gold_star_timestamp);
}

use std::fs;

fn parse_nums(input: &String) -> Vec<i64> {
    input
        .split("\n")
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

fn sum_exists(preamble: &[i64], sum: i64) -> bool {
    for i in preamble {
        if let Some(_) = preamble.iter().find(|&&j| j == (sum - *i)) {
            return true;
        }
    }

    return false;
}

fn find_invalid_num(nums: &Vec<i64>) -> i64 {
    for i in 25..nums.len() {
        let preamble = &nums[(i - 25)..i];
        if !sum_exists(&preamble, nums[i]) {
            return nums[i];
        }
    }

    -1
}

fn find_weakness(nums: &Vec<i64>, invalid_num: i64) -> Vec<i64> {
    for i in 0..nums.len() {
        let mut sum: i64 = nums[i];
        for j in (i + 1)..nums.len() {
            sum += nums[j];
            if sum == invalid_num {
                return nums[i..(j + 1)].to_vec();
            } else if sum > invalid_num {
                break;
            }
        }
    }

    vec![]
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let nums = parse_nums(&input);
    let invalid_num = find_invalid_num(&nums);
    println!("Invalid num: {}", invalid_num);

    let mut weakness = find_weakness(&nums, invalid_num);
    weakness.sort();
    let weakness = weakness[0] + weakness[weakness.len() - 1];
    println!("Weakness: {}", weakness);
}

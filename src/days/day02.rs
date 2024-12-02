use crate::util;

const DAY: &str = "02";

pub fn solve(use_example_input: bool) {
    let input = util::get_string_from_input(DAY, use_example_input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if is_report_safe(&nums) {
            safe_count += 1;
        }
    }

    safe_count
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            for index in 0..nums.len() {
                if is_report_safe(&[&nums[..index], &nums[index + 1..]].concat()) {
                    return true;
                }
            }

            false
        })
        .count() as i32
}

fn is_report_safe(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for pair in report.windows(2) {
        let diff = pair[1] - pair[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff > 0 {
            decreasing = false;
        } else if diff < 0 {
            increasing = false;
        }
    }

    increasing || decreasing
}

use crate::util;
use regex::Regex;

const DAY: &str = "03";

pub fn solve(use_example_input: bool) {
    let input = util::get_string_from_input(DAY, use_example_input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mult_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    mult_regex
        .captures_iter(input)
        .map(|caps| {
            let first = caps[1].parse::<i32>().unwrap();
            let second = caps[2].parse::<i32>().unwrap();
            first * second
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let input = input.replace('\n', " ").replace('\r', " ");
    let doable_part = Regex::new(r"(^.*?don't\(\))|(do\(\).*?(don't\(\)|$))").unwrap();
    doable_part
        .find_iter(&input)
        .map(|m| part1(&m.as_str()))
        .sum()
}

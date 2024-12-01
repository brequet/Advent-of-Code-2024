use crate::util;

const DAY: &str = "01";

pub fn solve(use_example_input: bool) {
    let input = util::get_string_from_input(DAY, use_example_input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let (mut left_list, mut right_list) = util::read_input_as_int_pairs(input);

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (left_list, right_list) = util::read_input_as_int_pairs(input);

    left_list
        .iter()
        .map(|&i| i * right_list.iter().filter(|&&x| x == i).count() as i32)
        .sum()
}

use std::collections::HashSet;

use crate::util;

const DAY: &str = "04";

pub fn solve(use_example_input: bool) {
    let input = util::get_string_from_input(DAY, use_example_input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let word = "XMAS";
    let matrix = util::Matrix::from_str(input);

    let mut count = 0;

    for r in 0..matrix.rows() {
        for c in 0..matrix.cols() {
            if matrix.data[r][c] == 'X' {
                count += check_word_every_dir(&matrix, r as i32, c as i32, word);
            }
        }
    }

    count
}

fn check_word_every_dir(matrix: &util::Matrix, r: i32, c: i32, word: &str) -> i32 {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (-1, -1), // Up-Left
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
    ];

    let mut count = 0;

    for &(dr, dc) in &directions {
        if check_word(matrix, r, c, dr, dc, word) {
            count += 1;
        }
    }

    count
}

fn check_word(matrix: &util::Matrix, r: i32, c: i32, dr: i32, dc: i32, word: &str) -> bool {
    let rows = matrix.rows() as i32;
    let cols = matrix.cols() as i32;
    let mut rr = r;
    let mut cc = c;

    for ch in word.chars() {
        if rr < 0
            || cc < 0
            || rr >= rows
            || cc >= cols
            || matrix.data[rr as usize][cc as usize] != ch
        {
            return false;
        }
        rr += dr;
        cc += dc;
    }

    true
}

fn part2(input: &str) -> i32 {
    let matrix = util::Matrix::from_str(input);

    let mut count = 0;

    for r in 0..matrix.rows() {
        for c in 0..matrix.cols() {
            if matrix.data[r][c] == 'A' && check_cross(&matrix, r as i32, c as i32) {
                count += 1;
            }
        }
    }

    count
}

fn check_cross(matrix: &util::Matrix, r: i32, c: i32) -> bool {
    let required_chars: HashSet<char> = HashSet::from(['M', 'S']);

    let chars_diag1: HashSet<char> =
        vec![matrix.get_char(r + 1, c + 1), matrix.get_char(r - 1, c - 1)]
            .into_iter()
            .flatten()
            .collect();

    let chars_diag2: HashSet<char> =
        vec![matrix.get_char(r + 1, c - 1), matrix.get_char(r - 1, c + 1)]
            .into_iter()
            .flatten()
            .collect();

    if chars_diag1 == required_chars && chars_diag2 == required_chars {
        true
    } else {
        false
    }
}

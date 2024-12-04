use crate::util;

const DAY: &str = "04";

pub fn solve(use_example_input: bool) {
    let input = util::get_string_from_input(DAY, use_example_input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Matrix(Vec<Vec<char>>);
// impl trait so matrix is printed like a grid, line by line
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self {
            for c in line {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> i32 {
    let matrix = Vec::from_iter(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>()),
    );

    let n = matrix.len();
    let m = matrix[0].len();

    println!("{:?}", matrix);

    0
}

fn part2(input: &str) -> i32 {
    0
}

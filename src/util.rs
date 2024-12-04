pub fn get_string_from_input(day_number: &str, use_example_input: bool) -> String {
    let input_file = format!(
        "inputs/day{}{}.txt",
        day_number,
        if use_example_input { ".ex" } else { "" }
    );

    std::fs::read_to_string(input_file).unwrap()
}

pub fn read_input_as_int_pairs(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let left_num = parts
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Invalid number");
        let right_num = parts
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Invalid number");

        left_list.push(left_num);
        right_list.push(right_num);
    }

    (left_list, right_list)
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<Vec<char>>,
}

impl Matrix {
    pub fn from_str(input: &str) -> Self {
        Matrix {
            data: input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect(),
        }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data.first().map_or(0, |row| row.len())
    }

    pub fn get_char(&self, r: i32, c: i32) -> Option<char> {
        let rows = self.rows() as i32;
        let cols = self.cols() as i32;

        if r < 0 || c < 0 || r >= rows || c >= cols {
            return None;
        }

        Some(self.data[r as usize][c as usize])
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for char in row {
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

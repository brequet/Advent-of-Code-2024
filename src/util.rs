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

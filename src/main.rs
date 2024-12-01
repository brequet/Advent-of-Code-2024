mod days;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day> [--example]", args[0]);
        return;
    }

    let use_example_input = args.contains(&String::from("--example"));

    let day = &args[1];
    match day.as_str() {
        "1" => days::day01::solve(use_example_input),
        _ => eprintln!("Unknown day: {}", day),
    }
}

use std::env;

const DEFAULT_PROBLEM: u32 = 26;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let answer = match args
        .iter()
        .nth(1)
        .map_or(DEFAULT_PROBLEM, |s| s.parse::<u32>().unwrap())
    {
        23 => projecteuler_rust::problem23::solve_str(),
        24 => projecteuler_rust::problem24::solve_str(),
        25 => projecteuler_rust::problem25::solve_str(),
        26 => projecteuler_rust::problem26::solve_str(),
        _ => unreachable!(),
    };
    println!("Answer: {}", answer);
}

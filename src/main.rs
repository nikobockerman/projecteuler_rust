use std::env;

const DEFAULT_PROBLEM: u32 = 25;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args
        .iter()
        .nth(1)
        .map_or(DEFAULT_PROBLEM, |s| s.parse::<u32>().unwrap())
    {
        23 => projecteuler_rust::problem23::print(),
        24 => projecteuler_rust::problem24::print(),
        25 => projecteuler_rust::problem25::print(),
        _ => unreachable!(),
    }
}

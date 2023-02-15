use std::env;

const DEFAULT_PROBLEM: u32 = 23;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args
        .iter()
        .nth(1)
        .map_or(DEFAULT_PROBLEM, |s| s.parse::<u32>().unwrap())
    {
        23 => projecteuler_rust::problem23::problem23(),
        _ => unreachable!(),
    }
}

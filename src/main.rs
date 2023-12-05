use aoc::days::*;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide the day to run as a command-line argument.");
    }

    let day = &args[1];

    let time = Instant::now();
    let res = solve_day(day);
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

    println!("\n=== Day {:02} ===", day);
    println!("  · Part 1: {}", res.0);
    println!("  · Part 2: {}", res.1);
    println!("  · Elapsed: {:.8} ms", elapsed_ms);
}

fn solve_day(day: &str) -> (usize, usize) {
    match day {
        "1" => day01::solve(),
        "2" => day02::solve(),
        "4" => day04::solve(),
        _ => unimplemented!(),
    }
}

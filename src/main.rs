mod days;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    let day: u32 = args[1].parse().unwrap_or(0);

    match day {
        1 => days::day01::solve(),
        _ => println!("Day not yet resolved!"),
    }
}

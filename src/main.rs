use std::io::{self, BufReader};

mod boj;
mod hacker_rank;
mod practice;
mod programmers;
mod utils;

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    boj::p1k::p1000::solve1000(&mut reader, &mut writer);
}

use std::io::Write;
use std::io::{self, BufWriter};

mod boj;
mod hacker_rank;
mod practice;
mod programmers;
mod utils;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let writer = stdout.lock();
    let mut writer = BufWriter::new(writer);

    boj::p1k::p1000::solve1000(&mut reader, &mut writer);

    writer.flush().expect("Flushing failed");
}

#![allow(
    clippy::collapsible_match,
    clippy::manual_is_multiple_of,
    clippy::needless_range_loop,
    clippy::unnecessary_sort_by,
    clippy::useless_vec
)]

mod boj;
mod hacker_rank;
mod practice;
mod programmers;
mod utils;

use std::io::Write;
use std::io::{self, BufWriter};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let writer = stdout.lock();
    let mut writer = BufWriter::new(writer);

    boj::p1k::p1000::solve1000(&mut reader, &mut writer);

    writer.flush().unwrap();
}

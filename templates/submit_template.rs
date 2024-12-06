use std::any::type_name;
use std::io::{self, BufWriter};
use std::io::{BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let writer = stdout.lock();
    let mut writer = BufWriter::new(writer);

    solve(&mut reader, &mut writer);

    writer.flush().expect("writer should be flushed");
}

// TODO: Implement solution function
// keep the suffix of the function name as the problem number
fn solve(reader: &mut impl BufRead, writer: &mut impl Write) {
    // Example of reading a single value
    let n: i32 = read_value(read_line(reader));

    // Example of reading multiple values from a line using read_values_as! macro
    let (a, b): (i32, i32) = read_values_as!(read_line(reader), i32, i32);

    // TODO: Implement solution logic
    let ans = n; // Placeholder

    // Write answer
    writeln!(writer, "{}", ans).expect("Failed to write");
}

// TODO: Add the sub function if needed
// fn sub_function() {
// }

// NOTE: The following utility functions are from src/utils/io.rs
// Copy only the functions you need and keep them in sync with the original implementation

// Required when using read_values_as! macro
#[macro_export]
macro_rules! read_values_as {
    ($line:expr, $( $t:ty ),+ ) => {{
        let line = $line;
        let mut iter = line.split_whitespace();
        (
            $(
                iter.next().expect("token should exist").parse::<$t>().expect("token should be parseable"),
            )+
        )
    }};
}

// Basic input functions - copy if needed
pub fn read_value<T: std::str::FromStr>(line: String) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    line.trim()
        .parse::<T>()
        .unwrap_or_else(|_| panic!("line should be parseable as {}", type_name::<T>()))
}

pub fn read_line(reader: &mut impl BufRead) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("line should be readable");
    line.trim().to_string()
}

// Additional utility functions - copy if needed:
//
// pub fn read_values<T: std::str::FromStr>(reader: &mut impl BufRead) -> Vec<T>
// pub fn read_n_values<T: std::str::FromStr>(reader: &mut impl BufRead, n: usize) -> Vec<T>
// pub fn read_map<T: std::str::FromStr>(reader: &mut impl BufRead, rows: usize, cols: usize) -> Vec<Vec<T>>
// pub fn matrix_to_str<T: ToString + std::fmt::Display>(mat: &[Vec<T>]) -> String
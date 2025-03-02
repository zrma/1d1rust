use std::any::type_name;
use std::io::BufRead;

pub fn read_value<T: std::str::FromStr>(line: String) -> T
where
    T::Err: std::fmt::Debug,
{
    let trimmed = line.trim();
    trimmed.parse::<T>().unwrap_or_else(|_| {
        panic!(
            "expected to parse a value of type `{}` from '{}', but parsing failed",
            type_name::<T>(),
            trimmed
        )
    })
}

pub fn read_line(reader: &mut impl BufRead) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("expected to read a line from input, but encountered an error");
    line.trim().to_string()
}

pub fn read_line_strict(reader: &mut impl BufRead) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line.pop(); // '\n'
        if line.ends_with('\r') {
            line.pop(); // '\r'
        }
    }
    line
}

#[macro_export]
macro_rules! read_values_as {
    ($line:expr, $( $t:ty ),+ ) => {{
        let ln = $line;
        let mut iter = ln.split_whitespace();
        (
            $(
                {
                    let token = iter
                        .next()
                        .expect("expected a token, but found none");
                    token.parse::<$t>()
                        .unwrap_or_else(|_| panic!(
                            "expected a value of type `{}` from '{}', but parsing failed",
                            stringify!($t),
                            token
                        ))
                },
            )+
        )
    }};
}

pub fn read_values<T: std::str::FromStr>(reader: &mut impl BufRead) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let line = read_line(reader);
    line.split_whitespace()
        .map(|s| {
            s.parse::<T>().unwrap_or_else(|_| {
                panic!(
                    "expected a value of type `{}` from '{}', but parsing failed",
                    type_name::<T>(),
                    s
                )
            })
        })
        .collect()
}

pub fn read_n_values<T: std::str::FromStr>(reader: &mut impl BufRead, n: usize) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let line = read_line(reader);
    let mut tokens = line.split_whitespace();
    (0..n)
        .map(|_| {
            let s = tokens
                .next()
                .expect("expected more tokens, but fewer tokens were provided");
            s.parse::<T>().unwrap_or_else(|_| {
                panic!(
                    "expected a value of type `{}` from '{}', but parsing failed",
                    type_name::<T>(),
                    s
                )
            })
        })
        .collect()
}

pub fn matrix_to_str<T: std::fmt::Display>(mat: &[Vec<T>]) -> String {
    mat.iter()
        .map(|row| {
            row.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn read_map<T: std::str::FromStr>(
    reader: &mut impl BufRead,
    rows: usize,
    cols: usize,
) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    (0..rows).map(|_| read_n_values(reader, cols)).collect()
}

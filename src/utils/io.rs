use std::any::type_name;
use std::io::BufRead;

/// Reads a line from the given reader and parses it into type `T`.
/// Panics if parsing fails, describing what was expected.
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

/// Reads a single line from the reader, trimming trailing whitespace.
/// Panics if reading fails, explaining what was expected.
pub fn read_line(reader: &mut impl BufRead) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("expected to read a line from input, but encountered an error");
    line.trim().to_string()
}

/// A macro that takes a line &str and parses whitespace-separated tokens into the specified types.
/// Panics if missing a token or parsing fails, explaining what was expected.
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

/// Reads an entire line and parses all whitespace-separated tokens into a Vec<T>.
/// Panics if parsing fails, clarifying the expected type.
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

/// Reads a single line and parses the first `n` whitespace-separated tokens into a Vec<T>.
/// Panics if parsing fails, stating the expectation.
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

/// Converts a matrix (Vec<Vec<T>>) into a string representation, each row joined by spaces.
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

/// Reads a 2D map of size `rows x cols` from the reader, parsing each line's values into T.
/// Panics if parsing fails through the called functions.
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

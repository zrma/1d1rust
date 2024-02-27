use std::io::BufRead;

pub fn read_value<T: std::str::FromStr>(line: String) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    line.trim().parse::<T>().unwrap()
}

pub fn read_line(reader: &mut impl BufRead) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

#[macro_export]
macro_rules! read_values_as {
    ($line:expr, $( $t:ty ),+ ) => {{
        let line = $line;
        let mut iter = line.split_whitespace();
        (
            $(
                iter.next().unwrap().parse::<$t>().unwrap(),
            )+
        )
    }};
}

pub fn read_values<T: std::str::FromStr>(reader: &mut impl BufRead) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_line(reader)
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

pub fn read_n_values<T: std::str::FromStr>(reader: &mut impl BufRead, n: usize) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_line(reader)
        .split_whitespace()
        .take(n)
        .map(|s| s.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

pub fn matrix_to_str<T: ToString + std::fmt::Display>(mat: &[Vec<T>]) -> String {
    mat.iter()
        .map(|row| {
            row.iter()
                .map(ToString::to_string)
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
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    (0..rows)
        .map(|_| {
            read_line(reader)
                .split_whitespace()
                .take(cols)
                .map(|s| s.parse::<T>().unwrap())
                .collect::<Vec<T>>()
        })
        .collect::<Vec<_>>()
}

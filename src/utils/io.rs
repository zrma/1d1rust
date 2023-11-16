use std::io::BufRead;

pub fn read_line(reader: &mut impl BufRead) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

pub fn read_value<T: std::str::FromStr>(line: String) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    line.trim().parse::<T>().unwrap()
}

#[macro_export]
macro_rules! read_values {
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

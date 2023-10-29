use std::io::BufRead;

pub fn read_line(reader: &mut impl BufRead) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
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

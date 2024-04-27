use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve26546(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value::<usize>(read_line(reader));
    let results = (0..num_cases)
        .map(|_| {
            let (input_string, start_idx, end_idx) =
                read_values_as!(read_line(reader), String, usize, usize);
            let chars = input_string.chars().collect::<Vec<char>>();
            chars[..start_idx]
                .iter()
                .chain(chars[end_idx..].iter())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", results).expect("Failed to write");
}

// https://www.acmicpc.net/problem/26546
// noinspection SpellCheckingInspection
// Reverse
#[test]
fn test_solve26546() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
COMPUTER 1 3
SCIENCE 3 7
RULES 3 4"
                .to_string(),
            want: "CPUTER
SCI
RULS"
                .to_string(),
        },
        TestData {
            s: "5
XYZ 0 0
XYZ 1 1
XYZ 2 2
XYZ 3 3
XYZ 0 3"
                .to_string(),
            want: "XYZ
XYZ
XYZ
XYZ
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve26546(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

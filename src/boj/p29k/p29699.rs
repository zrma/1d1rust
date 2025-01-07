use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29699(reader: &mut impl BufRead, writer: &mut impl Write) {
    const GIVEN: &str = "WelcomeToSMUPC";
    let input_index: usize = read_value(read_line(reader));

    let circular_index = (input_index + GIVEN.len() - 1) % GIVEN.len();
    let character = GIVEN.chars().nth(circular_index).unwrap();

    writeln!(writer, "{}", character).unwrap();
}

// https://www.acmicpc.net/problem/29699
// Welcome to SMUPC!
#[test]
fn test_solve29699() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2".to_string(),
            want: "e".to_string(),
        },
        TestData {
            s: "15".to_string(),
            want: "W".to_string(),
        },
        TestData {
            s: "140000".to_string(),
            want: "C".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve29699(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

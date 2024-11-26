use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18698(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_cases: usize = read_value(read_line(reader));

    let mut answers = Vec::with_capacity(num_of_cases);
    for _ in 0..num_of_cases {
        let s = read_line(reader);
        let consecutive_u = s.chars().take_while(|&c| c == 'U').count();
        answers.push(consecutive_u.to_string());
    }
    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/18698
// noinspection SpellCheckingInspection
// The Walking Adam
#[test]
fn test_solve18698() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
UUUDU
DDD
UU"
            .to_string(),
            want: "3
0
2"
            .to_string(),
        },
        TestData {
            s: "1
U"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve18698(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

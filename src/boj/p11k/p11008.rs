use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11008(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_cases: usize = read_value(read_line(reader));
    let mut results = Vec::with_capacity(num_of_cases);

    for _ in 0..num_of_cases {
        let (target, pattern) = read_values_as!(read_line(reader), String, String);

        let mut total_keystrokes = 0;
        let mut current_index = 0;
        while current_index < target.len() {
            if target[current_index..].starts_with(&pattern) {
                total_keystrokes += 1;
                current_index += pattern.len();
            } else {
                total_keystrokes += 1;
                current_index += 1;
            }
        }

        results.push(total_keystrokes.to_string());
    }

    write!(writer, "{}", results.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/11008
// noinspection SpellCheckingInspection
// 복붙의 달인
#[test]
fn test_solve11008() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
banana bana
asakusa sa"
                .to_string(),
            want: "3
5"
            .to_string(),
        },
        TestData {
            s: "1
baaaaaaaaaaa aaa"
                .to_string(),
            want: "6".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11008(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

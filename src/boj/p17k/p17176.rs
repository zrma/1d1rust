use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17176(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut symbol_counts = vec![0; 53];
    for symbol_code in read_n_values::<usize>(reader, n) {
        symbol_counts[symbol_code] += 1;
    }

    let mut char_counts = vec![0; 53];
    for char in read_line(reader).chars() {
        let index = match char {
            ' ' => 0,
            'A'..='Z' => (char as u8 - b'A' + 1) as usize,
            'a'..='z' => (char as u8 - b'a' + 27) as usize,
            _ => unreachable!(),
        };
        char_counts[index] += 1;
    }

    write!(
        writer,
        "{}",
        if symbol_counts == char_counts {
            "y"
        } else {
            "n"
        }
    )
    .expect("Failed to write");
}

// https://www.acmicpc.net/problem/17176
// noinspection SpellCheckingInspection
// 암호해독기
#[test]
fn test_solve17176() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "11
44 0 38 41 38 31 23 8 41 30 38
Hello World"
                .to_string(),
            want: "y".to_string(),
        },
        TestData {
            s: "5
12 3 34 52 0
apple"
                .to_string(),
            want: "n".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17176(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

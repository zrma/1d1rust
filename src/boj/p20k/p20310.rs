use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20310(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let (num_zeros, num_ones) = s.chars().fold((0, 0), |(zeros, ones), ch| {
        if ch == '0' {
            (zeros + 1, ones)
        } else {
            (zeros, ones + 1)
        }
    });

    let num_zeros_to_remove = num_zeros / 2;
    let num_ones_to_remove = num_ones / 2;

    fn filter_chars(s: &str, target: char, mut count: usize) -> String {
        s.chars()
            .filter(|&ch| {
                if ch == target && count > 0 {
                    count -= 1;
                    false
                } else {
                    true
                }
            })
            .collect()
    }

    let result = filter_chars(&s, '1', num_ones_to_remove);
    let result = filter_chars(
        &result.chars().rev().collect::<String>(),
        '0',
        num_zeros_to_remove,
    );
    let result = result.chars().rev().collect::<String>();

    write!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/20310
// 타노스
#[test]
fn test_solve20310() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1010".to_string(),
            want: "01".to_string(),
        },
        TestData {
            s: "11110000".to_string(),
            want: "1100".to_string(),
        },
        TestData {
            s: "10101010".to_string(),
            want: "0011".to_string(),
        },
        TestData {
            s: "000011".to_string(),
            want: "001".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20310(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

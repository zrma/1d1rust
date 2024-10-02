use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24510(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_cases: usize = read_value(read_line(reader));
    let ans = (0..num_of_cases)
        .map(|_| {
            let s = read_line(reader);
            let mut count = 0;
            let mut i = 0;
            while i < s.len() {
                if s[i..].starts_with("for") {
                    count += 1;
                    i += 3;
                } else if s[i..].starts_with("while") {
                    count += 1;
                    i += 5;
                } else {
                    i += 1;
                }
            }
            count
        })
        .max()
        .expect("Should have at least one case");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/24510
// noinspection SpellCheckingInspection
// 시간복잡도를 배운 도도
#[test]
fn test_solve24510() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
for
forwhileforfor
forwhileannsds"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "2
asdsdasds
dsdsdss"
                .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve24510(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

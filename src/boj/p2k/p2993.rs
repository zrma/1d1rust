use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2993(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut min = s.clone();

    for i in 1..s.len() {
        for j in i + 1..s.len() {
            let rev_a: String = s[..i].chars().rev().collect();
            let rev_b: String = s[i..j].chars().rev().collect();
            let rev_c: String = s[j..].chars().rev().collect();

            let combined = rev_a + &rev_b + &rev_c;
            if combined < min {
                min = combined;
            }
        }
    }

    write!(writer, "{}", min).expect("Failed to write");
}

// https://www.acmicpc.net/problem/2993
// noinspection SpellCheckingInspection
// 세 부분
#[test]
fn test_solve2993() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "dcbagfekjih".to_string(),
            want: "abcdefghijk".to_string(),
        },
        TestData {
            s: "mobitel".to_string(),
            want: "bometil".to_string(),
        },
        TestData {
            s: "anakonda".to_string(),
            want: "aanadnok".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2993(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

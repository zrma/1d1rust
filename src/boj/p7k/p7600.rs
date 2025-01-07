use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve7600(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let s = read_line(reader);
        if s == "#" {
            break;
        }

        let mut acc = [false; 26];
        for &byte in s.as_bytes() {
            if byte.is_ascii_alphabetic() {
                let idx = (byte.to_ascii_lowercase() - b'a') as usize;
                acc[idx] = true;
            }
        }

        let ans = acc.iter().filter(|&&x| x).count();
        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/7600
// 문자가 몇갤까
#[test]
fn test_solve7600() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "The quick brown fox jumped over the lazy dogs.
2 + 2 = 4
New Zealand Programming Contest.
#"
            .to_string(),
            want: "26
0
16
"
            .to_string(),
        },
        TestData {
            s: "The quick brown fox jumps over the lazy dog
#"
            .to_string(),
            want: "26
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve7600(&mut reader, &mut writer);

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

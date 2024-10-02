use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve7600(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let s = read_line(reader);
        if s == "#" {
            break;
        }

        let ans = s
            .as_bytes()
            .iter()
            .filter(|&x| x.is_ascii_alphabetic())
            .map(|&x| x.to_ascii_lowercase() - b'a')
            .fold([false; 26], |mut acc, x| {
                acc[x as usize] = true;
                acc
            })
            .iter()
            .filter(|&x| *x)
            .count();

        writeln!(writer, "{}", ans).expect("Failed to write");
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

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

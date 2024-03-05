use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12780(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let t = read_line(reader);

    let mut ans = 0;
    for i in 0..s.len() - t.len() + 1 {
        if s[i..i + t.len()] == t {
            ans += 1;
        }
    }

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/12780
// 원피스
// noinspection SpellCheckingInspection
#[test]
fn test_solve12780() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "INHAUNIVERSITYISONEOFTHEBESTUNIVERSITYINTHEWORLD
UNIVERSITY"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "INHAUNIVERSITYISONEOFTHEBESTUNIVERSITYINTHEWORLD
THE"
            .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve12780(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

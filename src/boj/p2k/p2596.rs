use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2596(reader: &mut impl BufRead, writer: &mut impl Write) {
    let _ = read_line(reader);

    let s = read_line(reader);
    let mut ans = String::new();

    for i in 0..s.len() / 6 {
        match decode(&s[i * 6..i * 6 + 6]) {
            Ok(v) => ans.push_str(v),
            Err(_) => {
                ans = (i + 1).to_string();
                break;
            }
        }
    }

    write!(writer, "{}", ans).unwrap();
}

fn decode(s: &str) -> Result<&str, &str> {
    let codes = vec![
        ("000000", "A"),
        ("001111", "B"),
        ("010011", "C"),
        ("011100", "D"),
        ("100110", "E"),
        ("101001", "F"),
        ("110101", "G"),
        ("111010", "H"),
    ];

    for (k, v) in codes {
        if s.chars().zip(k.chars()).filter(|(c, d)| c != d).count() < 2 {
            return Ok(v);
        }
    }
    Err("Invalid input")
}

// https://www.acmicpc.net/problem/2596
// 비밀편지
#[test]
fn test_solve2596() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "3
001111000000011100"
                .to_string(),
            want: "BAD".to_string(),
        },
        TestData {
            s: "5
011111000000111111000000111111"
                .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2596(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

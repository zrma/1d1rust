use crate::utils::io::read_line;
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2671(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let res = is_submarine(&line);

    write!(writer, "{}", if res { "SUBMARINE" } else { "NOISE" }).expect("Failed to write");
}

fn build_state_machine() -> HashMap<&'static str, (&'static str, &'static str)> {
    [
        ("START", ("0", "1")),
        ("1", ("10", "INVALID")),
        ("10", ("100+", "INVALID")),
        ("100+", ("100+", "100+1")),
        ("100+1", ("0", "100+1+")),
        ("100+1+", ("10|0", "100+1+")),
        ("0", ("INVALID", "01")),
        ("01", ("0", "1")),
        ("10|0", ("100+", "01")),
    ]
    .iter()
    .cloned()
    .collect()
}

fn is_submarine(s: &str) -> bool {
    let next = build_state_machine();
    let mut state = "START";

    for ch in s.chars() {
        let v = ch as usize - '0' as usize;
        state = if v == 0 { next[state].0 } else { next[state].1 };
        if state == "INVALID" {
            return false;
        }
    }

    matches!(state, "100+1" | "100+1+" | "01")
}

// https://www.acmicpc.net/problem/2671
// 잠수함식별
#[test]
fn test_solve2671() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10010111".to_string(),
            want: "NOISE".to_string(),
        },
        TestData {
            s: "100000000001101".to_string(),
            want: "SUBMARINE".to_string(),
        },
        TestData {
            s: "01100010011001".to_string(),
            want: "NOISE".to_string(),
        },
        TestData {
            s: "1010".to_string(),
            want: "NOISE".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2671(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

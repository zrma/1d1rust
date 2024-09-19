use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25192(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();

    let mut cnt = 0;
    let mut set = std::collections::HashSet::new();
    for _ in 0..n {
        let s = read_line(reader);
        if s == "ENTER" {
            set.clear();
        } else {
            let name = s;
            if !set.contains(&name) {
                set.insert(name);
                cnt += 1;
            }
        }
    }

    write!(writer, "{}", cnt).unwrap();
}

// https://www.acmicpc.net/problem/25192
// 인사성 밝은 곰곰이
// noinspection SpellCheckingInspection
#[test]
fn test_solve25192() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9
ENTER
pjshwa
chansol
chogahui05
lms0806
pichulia
r4pidstart
swoon
tony9402"
                .to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "7
ENTER
pjshwa
chansol
chogahui05
ENTER
pjshwa
chansol"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "3
ENTER
lms0806
lms0806"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25192(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

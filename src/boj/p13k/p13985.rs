use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13985(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);
    let nums: Vec<i32> = line
        .split([' ', '='])
        .filter_map(|s| s.parse().ok())
        .collect();

    let res = if nums[0] + nums[1] == nums[2] {
        "YES"
    } else {
        "NO"
    };

    write!(writer, "{}", res).expect("Failed to write");
}

// https://www.acmicpc.net/problem/13985
// Equality
#[test]
fn test_solve13985() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 + 2 = 3".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "3 + 2 = 2".to_string(),
            want: "NO".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13985(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

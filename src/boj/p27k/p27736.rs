use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27736(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let votes: Vec<i32> = read_n_values(reader, n);

    let (yes, no) = votes.iter().fold((0, 0), |(yes, no), &vote| match vote {
        1 => (yes + 1, no),
        -1 => (yes, no + 1),
        _ => (yes, no),
    });

    let ans = if yes + no <= n / 2 {
        "INVALID"
    } else if yes > no {
        "APPROVED"
    } else {
        "REJECTED"
    };

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/27736
// 찬반투표
#[test]
fn test_solve27736() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1 -1 1 1 -1"
                .to_string(),
            want: "APPROVED".to_string(),
        },
        TestData {
            s: "3
-1 1 -1"
                .to_string(),
            want: "REJECTED".to_string(),
        },
        TestData {
            s: "4
1 0 -1 1"
                .to_string(),
            want: "APPROVED".to_string(),
        },
        TestData {
            s: "4
1 0 0 1"
                .to_string(),
            want: "INVALID".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27736(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

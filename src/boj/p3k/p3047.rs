use crate::utils::io::{read_line, read_n_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3047(reader: &mut impl BufRead, writer: &mut impl Write) {
    let arr = {
        let mut temp_arr: Vec<i32> = read_n_values(reader, 3);
        temp_arr.sort_unstable();
        temp_arr
    };

    let sequence = read_line(reader);
    let ans = sequence
        .chars()
        .map(|c| match c {
            'A' => arr[0],
            'B' => arr[1],
            'C' => arr[2],
            _ => unreachable!(),
        })
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/3047
// ABC
#[test]
fn test_solve3047() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 5 3
ABC"
            .to_string(),
            want: "1 3 5".to_string(),
        },
        TestData {
            s: "1 2 3
ACB"
            .to_string(),
            want: "1 3 2".to_string(),
        },
        TestData {
            s: "1 2 3
BAC"
            .to_string(),
            want: "2 1 3".to_string(),
        },
        TestData {
            s: "1 2 3
BCA"
            .to_string(),
            want: "2 3 1".to_string(),
        },
        TestData {
            s: "6 4 2
CAB"
            .to_string(),
            want: "6 2 4".to_string(),
        },
        TestData {
            s: "1 2 3
CBA"
            .to_string(),
            want: "3 2 1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3047(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15916(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let arr = read_line(reader)
        .split_whitespace()
        .take(n)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let k: i64 = read_value(read_line(reader));

    let initial_cmp = arr[0].cmp(&k);
    if initial_cmp == std::cmp::Ordering::Equal {
        write!(writer, "T").expect("Failed to write");
        return;
    }

    for (i, &val) in arr.iter().enumerate().skip(1) {
        let target = k * (i as i64 + 1);

        if val.cmp(&target) == initial_cmp {
            continue;
        }

        write!(writer, "T").expect("Failed to write");
        return;
    }

    write!(writer, "F").expect("Failed to write");
}

// https://www.acmicpc.net/problem/15916
// 가희는 그래플러야!!
#[test]
fn test_solve15916() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
3 2
1"
            .to_string(),
            want: "T".to_string(),
        },
        TestData {
            s: "2
2 3
1"
            .to_string(),
            want: "F".to_string(),
        },
        TestData {
            s: "1
0
0"
            .to_string(),
            want: "T".to_string(),
        },
        TestData {
            s: "2
3 2
8"
            .to_string(),
            want: "F".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15916(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

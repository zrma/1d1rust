use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28014(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let heights: Vec<i32> = read_n_values(reader, n);

    let ans = heights
        .iter()
        .fold((0, 0), |(count, prev), &curr| {
            if curr >= prev {
                (count + 1, curr)
            } else {
                (count, curr)
            }
        })
        .0;

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/28014
// 첨탑 밀어서 부수기
#[test]
fn test_solve28014() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6
1 3 2 5 8 1"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "8
1 2 3 4 5 6 7 8"
                .to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "8
8 7 6 5 4 3 2 1"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "8
7 7 6 5 4 3 2 1"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve28014(&mut reader, &mut writer);

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

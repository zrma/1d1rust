use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18113(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k, m) = read_values_as!(read_line(reader), i32, i32, i32);

    let mut lengths = vec![];
    for _ in 0..n {
        let value = read_value(read_line(reader));
        if value > 2 * k {
            lengths.push(value - 2 * k);
        } else if 2 * k > value && value > k {
            lengths.push(value - k);
        }
    }

    let mut ans = -1;
    let mut left = 1;
    let mut right = *lengths.iter().max().unwrap_or(&0);
    while left <= right {
        let mid = (left + right) / 2;
        let count: i32 = lengths.iter().map(|&x| x / mid).sum();
        if count >= m {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/18113
// 그르다 김가놈
#[test]
fn test_solve18113() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 6 4
20
10
3"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3 8 1
16
7
8"
            .to_string(),
            want: "-1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18113(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

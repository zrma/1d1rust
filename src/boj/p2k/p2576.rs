use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2576(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (total_odd_sum, smallest_odd) = (0..7).fold((None, i32::MAX), |(acc, min_odd), _| {
        let number: i32 = read_value(read_line(reader));
        if number % 2 != 0 {
            let new_acc = Some(acc.unwrap_or(0) + number);
            let new_min_odd = min_odd.min(number);
            (new_acc, new_min_odd)
        } else {
            (acc, min_odd)
        }
    });

    match total_odd_sum {
        Some(sum) => write!(writer, "{}\n{}", sum, smallest_odd).expect("write! should work"),
        None => write!(writer, "-1").expect("write! should work"),
    }
}

// https://www.acmicpc.net/problem/2576
// 홀수
#[test]
fn test_solve2576() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "12
77
38
41
53
92
85"
            .to_string(),
            want: "256
41"
            .to_string(),
        },
        TestData {
            s: "2
4
20
32
6
10
8"
            .to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "7
1
2
3
4
5
6
7"
            .to_string(),
            want: "16
1"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2576(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

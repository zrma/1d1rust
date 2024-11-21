use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9713(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    for _ in 0..t {
        let n: i32 = read_value(read_line(reader));
        writeln!(writer, "{}", sum_of_odds(n)).expect("Failed to write");
    }
}

fn sum_of_odds(n: i32) -> i32 {
    debug_assert!(n % 2 == 1, "n must be odd");
    debug_assert!((1..=100).contains(&n), "n must be between 1 and 100");
    (1..=n).step_by(2).sum()
}

// https://www.acmicpc.net/problem/9713
// Sum of Odd Sequence
#[test]
fn test_solve9713() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10
1
3
5
7
9
11
13
15
17
19"
            .to_string(),
            want: "1
4
9
16
25
36
49
64
81
100"
            .to_string(),
        },
        TestData {
            s: "1
99"
            .to_string(),
            want: "2500".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9713(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1026(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let a_values = {
        let mut values: Vec<i32> = read_n_values(reader, n);
        values.sort_unstable();
        values
    };
    let b_values = {
        let mut values: Vec<i32> = read_n_values(reader, n);
        values.sort_unstable_by(|a, b| b.cmp(a));
        values
    };

    let ans: i32 = a_values
        .iter()
        .zip(b_values.iter())
        .map(|(a, b)| a * b)
        .sum();
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/1026
// 보물
#[test]
fn test_solve1026() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1 1 1 6 0
2 7 8 3 1"
                .to_string(),
            want: "18".to_string(),
        },
        TestData {
            s: "3
1 1 3
10 30 20"
                .to_string(),
            want: "80".to_string(),
        },
        TestData {
            s: "9
5 15 100 31 39 0 0 3 26
11 12 13 2 3 4 5 9 1"
                .to_string(),
            want: "528".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1026(&mut reader, &mut writer);

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

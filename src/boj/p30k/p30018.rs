use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve30018(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let a: Vec<u32> = read_n_values(reader, n);
    let b: Vec<u32> = read_n_values(reader, n);

    let ans = a
        .iter()
        .zip(&b)
        .fold(0, |acc, (a, b)| acc + b.saturating_sub(*a));

    writeln!(writer, "{}", ans).expect("writeln! should work");
}

// https://www.acmicpc.net/problem/30018
// 타슈
#[test]
fn test_solve30018() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
3 1 4 2
2 2 3 3"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3
1 1 5
4 2 1"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "2
2 1
2 1"
            .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve30018(&mut reader, &mut writer);

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

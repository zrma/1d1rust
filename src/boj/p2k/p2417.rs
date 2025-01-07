use crate::utils::io::{read_line, read_value};
use num::ToPrimitive;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2417(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: u64 = read_value(read_line(reader));
    let sqrt_n = integer_sqrt(n);

    let ans = if sqrt_n * sqrt_n >= n {
        sqrt_n
    } else {
        sqrt_n + 1
    };
    writeln!(writer, "{}", ans).unwrap();
}

fn integer_sqrt(n: u64) -> u64 {
    n.to_f64().unwrap().sqrt().to_u64().unwrap()
}

// https://www.acmicpc.net/problem/4158
// noinspection SpellCheckingInspection
// CD
#[test]
fn test_solve4158() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "122333444455555".to_string(),
            want: "11060446".to_string(),
        },
        TestData {
            s: "144".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "145".to_string(),
            want: "13".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2417(&mut reader, &mut writer);

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

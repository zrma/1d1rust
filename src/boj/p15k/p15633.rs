use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15633(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));

    let sum = (1..=n).filter(|&i| n % i == 0).sum::<i32>();

    let ans = sum * 5 - 24;
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/15633
// Fan Death
#[test]
fn test_solve15633() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "36".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15633(&mut reader, &mut writer);

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

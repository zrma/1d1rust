use crate::utils::io::{read_line, read_n_values, read_value};
use std::convert::TryFrom;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28061(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let lemons: Vec<u64> = read_n_values(reader, n);

    let ans = lemons
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &lemon)| lemon.saturating_sub(1 + u64::try_from(index).unwrap()))
        .max()
        .unwrap_or(0);

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/28061
// 레몬 따기
#[test]
fn test_solve28061() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
2 3 4"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "4
100 97 90 12"
                .to_string(),
            want: "96".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve28061(&mut reader, &mut writer);

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

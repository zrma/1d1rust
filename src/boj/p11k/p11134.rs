use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11134(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans = (0..num_cases)
        .map(|_| {
            let (n, c) = read_values_as!(read_line(reader), i32, i32);
            (n + c - 1) / c
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/11134
// 쿠키애호가
#[test]
fn test_solve11134() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
6 2
10 3"
                .to_string(),
            want: "3
4"
            .to_string(),
        },
        TestData {
            s: "1
7 2"
            .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11134(&mut reader, &mut writer);

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

use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11006(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let ans = (0..num_cases)
        .map(|_| {
            let (total_legs, num_chickens) = read_values_as!(read_line(reader), i32, i32);
            let one_legged = 2 * num_chickens - total_legs;
            let two_legged = num_chickens - one_legged;
            format!("{} {}", one_legged, two_legged)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/11006
// 남욱이의 닭장
#[test]
fn test_solve11006() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
5 3
7 5"
            .to_string(),
            want: "1 2
3 2"
            .to_string(),
        },
        TestData {
            s: "1
10 5"
                .to_string(),
            want: "0 5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11006(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

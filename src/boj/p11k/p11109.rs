use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11109(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let mut answers = Vec::with_capacity(num_cases);
    for _ in 0..num_cases {
        let (d, n, s, p) = read_values_as!(read_line(reader), i64, i64, i64, i64);
        let time_serial = n * s;
        let time_parallel = d + n * p;

        let answer = match time_serial.cmp(&time_parallel) {
            Less => "do not parallelize",
            Equal => "does not matter",
            Greater => "parallelize",
        };
        answers.push(answer);
    }

    writeln!(writer, "{}", answers.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/11109
// 괴짜 교수
#[test]
fn test_solve11109() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
10 2 3 2
20 5 8 2
0 2 1 1"
                .to_string(),
            want: "do not parallelize
parallelize
does not matter"
                .to_string(),
        },
        TestData {
            s: "1
50 50 2 1"
                .to_string(),
            want: "does not matter".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11109(&mut reader, &mut writer);

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

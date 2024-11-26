use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9325(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans = (0..num_cases)
        .map(|_| {
            let s: i32 = read_value(read_line(reader));
            let n: i32 = read_value(read_line(reader));
            let total = (0..n)
                .map(|_| {
                    let (q, p) = read_values_as!(read_line(reader), i32, i32);
                    q * p
                })
                .sum::<i32>()
                + s;
            total.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/9325
// 얼마?
#[test]
fn test_solve9325() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
10000
2
1 2000
3 400
50000
0"
            .to_string(),
            want: "13200
50000"
                .to_string(),
        },
        TestData {
            s: "1
100
3
1 100
2 200
3 300"
                .to_string(),
            want: "1500".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve9325(&mut reader, &mut writer);

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

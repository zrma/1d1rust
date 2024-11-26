use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2721(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let triangular_number = |n| n * (n + 1) / 2;

    let ans = (0..num_cases)
        .map(|_| {
            let n: usize = read_value(read_line(reader));
            (1..=n).map(|k| k * triangular_number(k + 1)).sum::<usize>()
        })
        .map(|sum| sum.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/1547
// noinspection SpellCheckingInspection
// 삼각수의 합
#[test]
fn test_solve1547() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
3
4
5
10"
            .to_string(),
            want: "45
105
210
2145"
                .to_string(),
        },
        TestData {
            s: "1
1"
            .to_string(),

            want: "3".to_string(),
        },
        TestData {
            s: "2
1
2"
            .to_string(),

            want: "3
15"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2721(&mut reader, &mut writer);

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

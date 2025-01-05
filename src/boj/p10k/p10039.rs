use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10039(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut sum = 0;
    for _ in 0..5 {
        let score = read_value(read_line(reader));
        sum += std::cmp::max(40, score);
    }
    writeln!(writer, "{}", sum / 5).unwrap();
}

// https://www.acmicpc.net/problem/10039
// 평균 점수
#[test]
fn test_solve10039() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10
65
100
30
95"
            .to_string(),
            want: "68".to_string(),
        },
        TestData {
            s: "100
100
100
100
100"
            .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "0
5
10
15
20"
            .to_string(),
            want: "40".to_string(),
        },
        TestData {
            s: "10
10
10
10
10"
            .to_string(),
            want: "40".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10039(&mut reader, &mut writer);

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

use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3004(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_value(read_line(reader));

    // 가로로 자르는 횟수 (floor)
    let row = n / 2;
    // 세로로 자르는 횟수 (ceiling)
    let col = (n + 1) / 2;

    let ans = (row + 1) * (col + 1);

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/3004
// 체스판 조각
#[test]
fn test_solve3004() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "2601".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3004(&mut reader, &mut writer);

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

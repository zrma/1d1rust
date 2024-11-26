use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1598(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = read_values_as!(read_line(reader), i32, i32);

    let (r1, c1) = ((a - 1) / 4, (a - 1) % 4);
    let (r2, c2) = ((b - 1) / 4, (b - 1) % 4);

    let dist = (r1 - r2).abs() + (c1 - c2).abs();
    write!(writer, "{}", dist).expect("write! should work");
}

// https://www.acmicpc.net/problem/1598
// 꼬리를 무는 숫자 나열
#[test]
fn test_solve1598() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "11 33".to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "1 2".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 5".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 6".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "1 11".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "11 31".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "11 39".to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "11 38".to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "11 40".to_string(),
            want: "8".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve1598(&mut reader, &mut writer);

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

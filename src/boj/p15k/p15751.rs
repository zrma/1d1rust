use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15751(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, x, y) = read_values_as!(read_line(reader), i32, i32, i32, i32);

    let ans = [
        a.abs_diff(b),                 // 직접 이동
        a.abs_diff(x) + b.abs_diff(y), // 워프: (a -> x = y -> b)
        a.abs_diff(y) + b.abs_diff(x), // 워프: (a -> y = x -> b)
    ]
    .iter()
    .copied()
    .min()
    .expect("ans should not be empty");

    writeln!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/15751
// Teleportation
#[test]
fn test_solve15751() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 10 8 2".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3 10 6 6".to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "3 10 5 12".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "3 10 1 12".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "3 10 1 1".to_string(),
            want: "7".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve15751(&mut reader, &mut writer);

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

use crate::read_values_as;
use crate::utils::io::read_line;
use num::ToPrimitive;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1297(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (d, h, w) = read_values_as!(read_line(reader), f64, f64, f64);

    let x = (d * d / (h * h + w * w)).sqrt();
    let (y, z) = (x * h, x * w);

    let y_i64 = y.to_i64().unwrap_or_else(|| {
        eprintln!("{} should be a valid i64 value.", y);
        0
    });
    let z_i64 = z.to_i64().unwrap_or_else(|| {
        eprintln!("{} should be a valid i64 value.", z);
        0
    });

    write!(writer, "{} {}", y_i64, z_i64).expect("write! should work");
}

// https://www.acmicpc.net/problem/1297
// TV 크기
// noinspection SpellCheckingInspection
#[test]
fn test_solve1297() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "52 9 16".to_string(),
            want: "25 45".to_string(),
        },
        TestData {
            s: "7 2 3".to_string(),
            want: "3 5".to_string(),
        },
        TestData {
            s: "5 3 4".to_string(),
            want: "3 4".to_string(),
        },
        TestData {
            s: "13 5 12".to_string(),
            want: "5 12".to_string(),
        },
        TestData {
            s: "13 7 10".to_string(),
            want: "7 10".to_string(),
        },
        TestData {
            s: "7 32 47".to_string(),
            want: "3 5".to_string(),
        },
        TestData {
            s: "11 15 16".to_string(),
            want: "7 8".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1297(&mut reader, &mut writer);

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

use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve19532(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c, d, e, f) = read_values_as!(read_line(reader), i64, i64, i64, i64, i64, i64);

    // ax + by = c
    // dx + ey = f
    //
    // aex + bey = ce
    // bdx + bey = bf
    // (ae - bd) x = ce - bf
    // x = (ce - bf) / (ae - bd)
    //
    // adx + bdy = cd
    // adx + aey = af
    // (bd - ae) y = cd - af
    // y = (cd - af) / (bd - ae)

    let x = (c * e - b * f) / (a * e - b * d);
    let y = (c * d - a * f) / (b * d - a * e);

    write!(writer, "{} {}", x, y).expect("Failed to write");
}

#[allow(dead_code)]
fn solve19532_brute_force(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c, d, e, f) = read_values_as!(read_line(reader), i64, i64, i64, i64, i64, i64);

    for x in -999..=999 {
        for y in -999..=999 {
            if a * x + b * y == c && d * x + e * y == f {
                write!(writer, "{} {}", x, y).unwrap();
                return;
            }
        }
    }
}

// https://www.acmicpc.net/problem/19532
// 수학은 비대면강의입니다
#[test]
fn test_solve19532() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 3 -1 4 1 7".to_string(),
            want: "2 -1".to_string(),
        },
        TestData {
            s: "2 5 8 3 -4 -11".to_string(),
            want: "-1 2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve19532(&mut reader, &mut writer);

            let got = String::from_utf8(writer).expect("writer should be a valid string");
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }

        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve19532_brute_force(&mut reader, &mut writer);

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
}

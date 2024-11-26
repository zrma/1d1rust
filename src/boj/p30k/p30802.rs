use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve30802(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_value(read_line(reader));
    let (s, m, l, xl, xxl, xxxl) = read_values_as!(read_line(reader), i64, i64, i64, i64, i64, i64);
    let (t, p) = read_values_as!(read_line(reader), i64, i64);

    fn calculate_pieces(count: i64, t: i64) -> i64 {
        (count + t - 1) / t
    }

    let total_pieces = calculate_pieces(s, t)
        + calculate_pieces(m, t)
        + calculate_pieces(l, t)
        + calculate_pieces(xl, t)
        + calculate_pieces(xxl, t)
        + calculate_pieces(xxxl, t);

    let (full_packages, remaining_items) = (n / p, n % p);

    writeln!(writer, "{}", total_pieces).expect("Failed to write");
    writeln!(writer, "{} {}", full_packages, remaining_items).expect("Failed to write");
}

// https://www.acmicpc.net/problem/30802
// 웰컴 키트
#[test]
fn test_solve30802() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "23
3 1 4 1 5 9
5 7"
            .to_string(),
            want: "7
3 2"
            .to_string(),
        },
        TestData {
            s: "1000000000
1 999999999 0 0 0 0
1 1000000000"
                .to_string(),
            want: "1000000000
1 0"
            .to_string(),
        },
        TestData {
            s: "1
0 0 0 0 0 1
2 2"
            .to_string(),
            want: "1
0 1"
            .to_string(),
        },
        TestData {
            s: "33
13 1 4 1 5 9
5 7"
            .to_string(),
            want: "9
4 5"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve30802(&mut reader, &mut writer);

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

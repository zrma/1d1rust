use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2863(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (mut a, mut b) = read_values_as!(read_line(reader), f64, f64);
    let (mut c, mut d) = read_values_as!(read_line(reader), f64, f64);

    let mut max_val = calculate_expression(a, b, c, d);
    let mut max_idx = 0;

    for i in 1..4 {
        (a, b, c, d) = rotate(a, b, c, d);
        let curr_val = calculate_expression(a, b, c, d);
        if curr_val > max_val {
            max_val = curr_val;
            max_idx = i;
        }
    }

    write!(writer, "{}", max_idx).expect("Failed to write");
}

fn rotate(a: f64, b: f64, c: f64, d: f64) -> (f64, f64, f64, f64) {
    (c, a, d, b)
}

fn calculate_expression(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a / c + b / d
}

// https://www.acmicpc.net/problem/2863
// 이게 분수?
#[test]
fn test_solve2863() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2
3 4"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2 4
1 3"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5 9
7 2"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "41 99
100 13"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2863(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

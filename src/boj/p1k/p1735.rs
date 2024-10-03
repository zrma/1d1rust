use crate::read_values_as;
use crate::utils::io::read_line;
use num::integer::gcd;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1735(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = read_values_as!(read_line(reader), i64, i64);
    let (c, d) = read_values_as!(read_line(reader), i64, i64);

    let (x, y) = sum_and_reduce_fractions(a, b, c, d);
    write!(writer, "{} {}", x, y).expect("write! should work");
}

fn sum_and_reduce_fractions(a: i64, b: i64, c: i64, d: i64) -> (i64, i64) {
    let (mut numerator, mut denominator) = (a * d + b * c, b * d);
    let mut gcd_val = gcd(numerator, denominator);
    while gcd_val != 1 {
        numerator /= gcd_val;
        denominator /= gcd_val;
        gcd_val = gcd(numerator, denominator);
    }
    (numerator, denominator)
}

// https://www.acmicpc.net/problem/1735
// 분수 합
#[test]
fn test_solve1735() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "2 7
3 5"
        .to_string(),
        want: "31 35".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1735(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

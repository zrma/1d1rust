use crate::utils::io::read_line;
use num::integer::gcd;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1735(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = read_two_numbers(reader);
    let (c, d) = read_two_numbers(reader);

    let (mut x, mut y) = (a * d + b * c, b * d);
    let mut gcd_val = gcd(x, y);
    while gcd_val != 1 {
        x /= gcd_val;
        y /= gcd_val;
        gcd_val = gcd(x, y);
    }

    write!(writer, "{} {}", x, y).unwrap();
}

fn read_two_numbers(reader: &mut impl BufRead) -> (i64, i64) {
    let line = read_line(reader);
    let mut iter = line.split_whitespace();
    (
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
    )
}

// https://www.acmicpc.net/problem/1735
// 분수 합
#[test]
fn test_solve1735() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

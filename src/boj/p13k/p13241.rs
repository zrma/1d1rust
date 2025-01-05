use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13241(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = read_values_as!(read_line(reader), i64, i64);

    let res = lcm(a, b);
    writeln!(writer, "{}", res).unwrap();
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

// https://www.acmicpc.net/problem/13241
// 최소공배수
#[test]
fn test_solve13241() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3 5".to_string(),
            want: "15".to_string(),
        },
        TestData {
            s: "1 123".to_string(),
            want: "123".to_string(),
        },
        TestData {
            s: "121 199".to_string(),
            want: "24079".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13241(&mut reader, &mut writer);

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

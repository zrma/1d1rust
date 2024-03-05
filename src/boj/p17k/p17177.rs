use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17177(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c) = read_values_as!(read_line(reader), i32, i32, i32);

    // case 1: x, b, c
    // p^2 = a^2 - x^2
    // q^2 = a^2 - c^2
    // ab + xc = pq

    // case 2: b, x, c
    // p^2 = a^2 - b^2
    // q^2 = a^2 - c^2
    // ax + bc = pq

    // case 3: b, c, x
    // p^2 = a^2 - b^2
    // q^2 = a^2 - x^2
    // ac + bx = pq
    fn check(a: i32, b: i32, c: i32, x: i32) -> bool {
        let p_squared = a * a - b * b;
        let q_squared = a * a - c * c;
        let pq_squared = p_squared * q_squared;
        let ab_xc = a * b + x * c;
        let ax_bc = a * x + b * c;
        let ac_bx = a * c + b * x;

        ab_xc.pow(2) == pq_squared || ax_bc.pow(2) == pq_squared || ac_bx.pow(2) == pq_squared
    }

    for x in 1..a {
        if check(a, b, c, x) {
            write!(writer, "{}", x).expect("Failed to write");
            return;
        }
    }

    write!(writer, "-1").expect("Failed to write");
}

// https://www.acmicpc.net/problem/17177
// 내접사각형 만들기
#[test]
fn test_solve17177() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 1 1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 1 1".to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "72 42 23".to_string(),
            want: "42".to_string(),
        },
        TestData {
            s: "72 42 42".to_string(),
            want: "23".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17177(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

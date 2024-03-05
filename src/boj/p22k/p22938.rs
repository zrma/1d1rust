use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve22938(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (ax, ay, ar) = read_values_as!(read_line(reader), i64, i64, i64);
    let (bx, by, br) = read_values_as!(read_line(reader), i64, i64, i64);

    let res = is_intersect((ax, ay, ar), (bx, by, br));
    write!(writer, "{}", res).unwrap();
}

type Circle = (i64, i64, i64);

fn is_intersect(circle1: Circle, circle2: Circle) -> &'static str {
    const DISJOINT: &str = "NO";
    const INTERSECT: &str = "YES";

    let (ax, ay, ar) = circle1;
    let (bx, by, br) = circle2;

    let dx = ax - bx;
    let dy = ay - by;
    let d = dx.pow(2) + dy.pow(2);

    if d >= (ar + br).pow(2) {
        return DISJOINT;
    }

    INTERSECT
}

// https://www.acmicpc.net/problem/22938
// 백발백준하는 명사수
#[test]
fn test_solve22938() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0 0 1
1 1 2"
                .to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "0 0 1
2 0 1"
                .to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "0 0 1
10 10 1"
                .to_string(),
            want: "NO".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve22938(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

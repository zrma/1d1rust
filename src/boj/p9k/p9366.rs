use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9366(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();
    for i in 0..t {
        let (a, b, c) = read_values_as!(read_line(reader), i32, i32, i32);

        let res = triangle_type(a, b, c);
        writeln!(writer, "Case #{}: {}", i + 1, res).unwrap();
    }
}

fn triangle_type(p0: i32, p1: i32, p2: i32) -> String {
    let mut v = [p0, p1, p2];
    v.sort();
    let (a, b, c) = (v[0], v[1], v[2]);

    if a + b <= c {
        return "invalid!".to_string();
    }

    if a == b && b == c {
        return "equilateral".to_string();
    }

    if a == b || b == c {
        return "isosceles".to_string();
    }

    "scalene".to_string()
}

// https://www.acmicpc.net/problem/9366
// 삼각형 분류
#[test]
fn test_solve9366() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
3 3 4
6 4 2"
                .to_string(),
            want: "Case #1: isosceles
Case #2: invalid!
"
            .to_string(),
        },
        TestData {
            s: "3
3 4 5
4 4 4
4 3 2"
                .to_string(),
            want: "Case #1: scalene
Case #2: equilateral
Case #3: scalene
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9366(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

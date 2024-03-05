use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5073(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let mut sides = read_sides(reader);
        sides.sort();

        if sides[0] == 0 {
            break;
        }

        let res = classify_triangle(&sides);

        writeln!(writer, "{}", res).expect("Failed to write");
    }
}

fn read_sides(reader: &mut (impl BufRead + Sized)) -> Vec<i32> {
    read_line(reader)
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn classify_triangle(v: &[i32]) -> String {
    let a = v[0];
    let b = v[1];
    let c = v[2];

    if a == b && b == c {
        "Equilateral".to_string()
    } else if a + b <= c {
        "Invalid".to_string()
    } else if a == b || b == c || c == a {
        "Isosceles".to_string()
    } else {
        "Scalene".to_string()
    }
}

// https://www.acmicpc.net/problem/5073
// 삼각형과 세 변
#[test]
fn test_solve5073() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "7 7 7
6 5 4
3 2 5
6 2 6
0 0 0"
            .to_string(),
        want: "Equilateral
Scalene
Invalid
Isosceles
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5073(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

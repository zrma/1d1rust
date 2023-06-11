use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10101(reader: &mut impl BufRead, writer: &mut impl Write) {
    let angles = read_angles(reader);

    let res = classify_triangle(&angles);

    write!(writer, "{}", res).unwrap();
}

fn classify_triangle(p0: &(u32, u32, u32)) -> String {
    let (a, b, c) = p0;

    if a + b + c != 180 {
        "Error".to_string()
    } else if a == b && b == c {
        "Equilateral".to_string()
    } else if a == b || b == c || c == a {
        "Isosceles".to_string()
    } else {
        "Scalene".to_string()
    }
}

fn read_angles(p0: &mut (impl BufRead + Sized)) -> (u32, u32, u32) {
    let a = read_line(p0).parse::<u32>().unwrap();
    let b = read_line(p0).parse::<u32>().unwrap();
    let c = read_line(p0).parse::<u32>().unwrap();

    (a, b, c)
}

// https://www.acmicpc.net/problem/10101
// 삼각형 외우기
#[test]
fn test_solve10101() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "60
70
50"
            .to_string(),
            want: "Scalene".to_string(),
        },
        TestData {
            s: "30
30
120"
            .to_string(),
            want: "Isosceles".to_string(),
        },
        TestData {
            s: "60
60
60"
            .to_string(),
            want: "Equilateral".to_string(),
        },
        TestData {
            s: "60
70
80"
            .to_string(),
            want: "Error".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10101(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

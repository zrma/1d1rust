use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15803(reader: &mut impl BufRead, writer: &mut impl Write) {
    let points = (0..3)
        .map(|_| {
            let s = read_line(reader);
            let mut iter = s.split_whitespace();
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let (x1, y1) = points[0];
    let (x2, y2) = points[1];
    let (x3, y3) = points[2];

    let ans = is_line(x1, y1, x2, y2, x3, y3);
    write!(writer, "{}", ans).unwrap();
}

fn is_line(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> String {
    if (x1 - x2) * (y2 - y3) == (x2 - x3) * (y1 - y2) {
        "WHERE IS MY CHICKEN?".to_string()
    } else {
        "WINNER WINNER CHICKEN DINNER!".to_string()
    }
}

// https://www.acmicpc.net/problem/15803
// PLAYERJINAH’S BOTTLEGROUNDS
#[test]
fn test_solve15803() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 1
1 2
2 1"
            .to_string(),
            want: "WINNER WINNER CHICKEN DINNER!".to_string(),
        },
        TestData {
            s: "12 10
24 20
36 30"
                .to_string(),
            want: "WHERE IS MY CHICKEN?".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15803(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}

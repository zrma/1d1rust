use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14400(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    let mut points = vec![];
    for _ in 0..n {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let x = iter.next().unwrap().parse::<i64>().unwrap();
        let y = iter.next().unwrap().parse::<i64>().unwrap();
        points.push((x, y));
    }

    points.sort_by(|a, b| a.0.cmp(&b.0));
    let median_x = points[n / 2].0;

    points.sort_by(|a, b| a.1.cmp(&b.1));
    let median_y = points[n / 2].1;

    let mut ans = 0;
    for (x, y) in points {
        ans += (x - median_x).abs() + (y - median_y).abs();
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/14400
// 편의점 2
#[test]
fn test_solve14400() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
2 2
3 4
5 6
1 9
-2 -8"
                .to_string(),
            want: "30".to_string(),
        },
        TestData {
            s: "1
1 1"
            .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14400(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
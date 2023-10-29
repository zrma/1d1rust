use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15923(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let mut points = vec![];
    for _ in 0..n {
        points.push(read_values!(read_line(reader), i32, i32));
    }

    let ans = points
        .iter()
        .zip(points.iter().skip(1))
        .map(|((x1, y1), (x2, y2))| (x2 - x1).abs() + (y2 - y1).abs())
        .sum::<i32>()
        + (points[0].0 - points[n - 1].0).abs()
        + (points[0].1 - points[n - 1].1).abs();

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/15923
// 욱제는 건축왕이야!!
#[test]
fn test_solve15923() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
10 10
10 40
40 40
40 10"
                .to_string(),
            want: "120".to_string(),
        },
        TestData {
            s: "8
0 0
6 0
6 6
4 6
4 8
2 8
2 6
0 6"
            .to_string(),
            want: "28".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15923(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

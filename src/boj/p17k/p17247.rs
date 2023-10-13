use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17247(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        )
    };

    let (mut x1, mut y1, mut x2, mut y2) = (0, 0, 0, 0);

    let mut found = false;

    for i in 0..n {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        for j in 0..m {
            let v = iter.next().unwrap().parse::<i32>().unwrap();
            if v == 1 {
                if !found {
                    x1 = i;
                    y1 = j;
                    found = true;
                } else {
                    x2 = i;
                    y2 = j;
                }
            }
        }
    }

    let ans = (x1 - x2).abs() + (y1 - y2).abs();
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/17247
// 택시 거리
#[test]
fn test_solve17247() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 4
1 0 0 0
0 0 0 0
0 0 0 1"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "4 4
1 0 0 0
0 0 0 0
0 0 0 0
0 0 0 1"
                .to_string(),
            want: "6".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17247(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}

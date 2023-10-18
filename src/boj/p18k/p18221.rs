use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18221(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let mut a = vec![vec![0; n]; n];

    let (mut x0, mut y0) = (0, 0);
    let (mut x1, mut y1) = (0, 0);

    for y in 0..n {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        for x in 0..n {
            let v = iter.next().unwrap().parse::<i32>().unwrap();
            a[y][x] = v;

            if v == 2 {
                x0 = x;
                y0 = y;
            } else if v == 5 {
                x1 = x;
                y1 = y;
            }
        }
    }

    if (x0 - x1) * (x0 - x1) + (y0 - y1) * (y0 - y1) < 25 {
        write!(writer, "0").unwrap();
        return;
    }

    let mut cnt = 0;
    for y in y0.min(y1)..=y0.max(y1) {
        for x in x0.min(x1)..=x0.max(x1) {
            if a[y][x] == 1 {
                cnt += 1;

                if cnt > 2 {
                    write!(writer, "1").unwrap();
                    return;
                }
            }
        }
    }

    write!(writer, "0").unwrap();
}

// https://www.acmicpc.net/problem/18221
// 교수님 저는 취업할래요
#[test]
fn test_solve18221() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
0 5 0 0 0 0 0
0 0 1 0 0 0 0
0 0 0 0 0 0 0
0 0 0 1 1 0 0
0 0 0 0 0 2 0
0 0 0 0 0 0 0
0 0 0 0 0 0 0"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "9
0 5 0 0 0 0 0 0 0
0 1 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0
0 0 1 1 0 0 0 0 0
1 0 0 0 0 0 0 0 0
0 0 2 0 1 0 0 0 0
0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "10
0 5 0 0 0 0 1 0 0 0
0 1 0 0 0 0 1 0 0 0
0 0 0 0 0 0 2 0 0 0
0 0 1 1 0 0 1 0 0 0
1 0 0 0 0 0 0 0 0 0
0 0 1 0 1 0 0 0 0 0
0 0 0 0 0 0 1 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "9
0 5 0 0 0 0 0 0 0
0 1 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0
0 0 1 1 0 0 0 0 0
1 0 0 0 0 0 0 0 0
0 2 0 0 1 0 0 0 0
0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "9
0 5 0 0 0 0 0 0 0
0 1 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0
0 1 1 1 0 0 0 0 0
1 1 0 0 0 0 0 0 0
0 2 0 0 1 0 0 0 0
0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18221(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}

use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18221(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();
    let mut arr = vec![vec![0; n]; n];

    let (mut x0, mut y0) = (0, 0);
    let (mut x1, mut y1) = (0, 0);

    arr.iter_mut().enumerate().for_each(|(y, row)| {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        row.iter_mut().enumerate().for_each(|(x, col)| {
            let v: i32 = iter.next().unwrap().parse().unwrap();
            *col = v;

            if v == 2 {
                x0 = x;
                y0 = y;
            } else if v == 5 {
                x1 = x;
                y1 = y;
            }
        });
    });

    if (x0 - x1) * (x0 - x1) + (y0 - y1) * (y0 - y1) < 25 {
        write!(writer, "0").expect("Failed to write");
        return;
    }

    let mut cnt = 0;
    for row in arr.iter().take(y0.max(y1) + 1).skip(y0.min(y1)) {
        for &val in row.iter().take(x0.max(x1) + 1).skip(x0.min(x1)) {
            if val == 1 {
                cnt += 1;
                if cnt > 2 {
                    write!(writer, "1").expect("Failed to write");
                    return;
                }
            }
        }
    }

    write!(writer, "0").expect("Failed to write");
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

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

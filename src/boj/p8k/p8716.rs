use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8716(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (x1, y1, x2, y2) = read_values!(read_line(reader), i64, i64, i64, i64);
    let (x3, y3, x4, y4) = read_values!(read_line(reader), i64, i64, i64, i64);

    let horizontal_line1 = Line::new(x1, x2);
    let horizontal_line2 = Line::new(x3, x4);
    let vertical_line1 = Line::new(y1, y2);
    let vertical_line2 = Line::new(y3, y4);

    let x_intersect = get_intersect(horizontal_line1, horizontal_line2);
    let y_intersect = get_intersect(vertical_line1, vertical_line2);

    write!(writer, "{}", x_intersect * y_intersect).unwrap();
}

struct Line {
    lo: i64,
    hi: i64,
}

impl Line {
    fn new(a: i64, b: i64) -> Self {
        Line {
            lo: a.min(b),
            hi: a.max(b),
        }
    }
}

fn get_intersect(line1: Line, line2: Line) -> i64 {
    if line1.lo > line2.hi || line1.hi < line2.lo {
        0
    } else {
        (line1.hi.min(line2.hi) - line1.lo.max(line2.lo)).max(0)
    }
}

// https://www.acmicpc.net/problem/8716
// Pole
#[test]
fn test_solve8716() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0 3 4 0
2 4 6 1"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "0 4 4 1
2 3 6 0"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "0 5 5 0
1 4 4 1"
                .to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "0 5 5 0
1 6 2 2"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "0 2 2 0
1 1 2 -1"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "0 2 2 0
1 2 2 1"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "0 200000 200000 0
100000 200000 200000 100000"
                .to_string(),
            want: "10000000000".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8716(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

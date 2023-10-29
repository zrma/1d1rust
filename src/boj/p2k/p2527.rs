use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2527(reader: &mut impl BufRead, writer: &mut impl Write) {
    for _ in 0..4 {
        let (ax, ay, ap, aq, bx, by, bp, bq) = read_values!(
            &read_line(reader),
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize
        );

        let res = common_point_code((ax, ay, ap, aq), (bx, by, bp, bq));
        writeln!(writer, "{}", res).unwrap();
    }
}

type Rect = (usize, usize, usize, usize);

fn common_point_code(rect1: Rect, rect2: Rect) -> char {
    const DISJOINT: char = 'd';
    const COMMON_POINT: char = 'c';
    const COMMON_LINE: char = 'b';
    const COMMON_AREA: char = 'a';

    let (ax, ay, ap, aq) = rect1;
    let (bx, by, bp, bq) = rect2;

    if ax > bp || ay > bq || bx > ap || by > aq {
        return DISJOINT;
    }

    if (ax == bp || ap == bx) && (ay == bq || aq == by) {
        return COMMON_POINT;
    }

    if (ax == bp || ap == bx) || (ay == bq || aq == by) {
        return COMMON_LINE;
    }

    COMMON_AREA
}

// https://www.acmicpc.net/problem/2527
// 직사각형
#[test]
fn test_solve2527() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 10 50 60 100 100 200 300
45 50 600 600 400 450 500 543
11 120 120 230 50 40 60 440
35 56 67 90 67 80 500 600"
                .to_string(),
            want: "d
a
a
b
"
            .to_string(),
        },
        TestData {
            s: "2 2 3 3 3 3 4 4
2 2 3 3 1 1 2 2
2 2 3 3 3 1 4 2
2 2 3 3 1 3 2 4"
                .to_string(),
            want: "c
c
c
c
"
            .to_string(),
        },
        TestData {
            s: "2 2 3 3 3 3 4 4
2 2 3 3 1 1 2 2
2 2 3 3 3 1 4 2
5 2 8 7 1 3 5 4"
                .to_string(),
            want: "c
c
c
b
"
            .to_string(),
        },
        TestData {
            s: "10 10 20 20 21 21 30 30
10 10 20 20 10 10 20 20
10 10 20 20 20 10 30 20
10 10 20 20 10 20 20 30"
                .to_string(),
            want: "d
a
b
b
"
            .to_string(),
        },
        TestData {
            s: "10 10 20 20 20 20 30 30
10 10 20 20 20 19 30 30
10 10 20 20 20 10 30 19
10 10 20 20 10 20 19 30"
                .to_string(),
            want: "c
b
b
b
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2527(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15973(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (ax, ay, ap, aq) = read_values_as!(read_line(reader), i32, i32, i32, i32);
    let (bx, by, bp, bq) = read_values_as!(read_line(reader), i32, i32, i32, i32);

    let res = common_point_code((ax, ay, ap, aq), (bx, by, bp, bq));
    write!(writer, "{}", res).unwrap();
}

type Rect = (i32, i32, i32, i32);

fn common_point_code(rect1: Rect, rect2: Rect) -> String {
    const DISJOINT: &str = "NULL";
    const COMMON_POINT: &str = "POINT";
    const COMMON_LINE: &str = "LINE";
    const COMMON_AREA: &str = "FACE";

    let (ax, ay, ap, aq) = rect1;
    let (bx, by, bp, bq) = rect2;

    if ax > bp || ay > bq || bx > ap || by > aq {
        return DISJOINT.to_string();
    }

    if (ax == bp || ap == bx) && (ay == bq || aq == by) {
        return COMMON_POINT.to_string();
    }

    if (ax == bp || ap == bx) || (ay == bq || aq == by) {
        return COMMON_LINE.to_string();
    }

    COMMON_AREA.to_string()
}

// https://www.acmicpc.net/problem/15973
// 두 박스
#[test]
fn test_solve15973() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2 3 5
3 5 8 11"
                .to_string(),
            want: "POINT".to_string(),
        },
        TestData {
            s: "3 5 9 10
5 4 8 11"
                .to_string(),
            want: "FACE".to_string(),
        },
        TestData {
            s: "1 2 3 5
9 3 12 7"
                .to_string(),
            want: "NULL".to_string(),
        },
        TestData {
            s: "3 5 9 10
9 3 12 7"
                .to_string(),
            want: "LINE".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15973(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

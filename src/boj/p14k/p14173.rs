use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14173(reader: &mut impl BufRead, writer: &mut impl Write) {
    let rect1 = {
        let (x1, y1, x2, y2) = read_values_as!(read_line(reader), i32, i32, i32, i32);
        Rectangle { x1, y1, x2, y2 }
    };

    let rect2 = {
        let (x3, y3, x4, y4) = read_values_as!(read_line(reader), i32, i32, i32, i32);
        Rectangle {
            x1: x3,
            y1: y3,
            x2: x4,
            y2: y4,
        }
    };

    let (min_x, min_y, max_x, max_y) = bounding_box(&rect1, &rect2);

    let side = (max_x - min_x).max(max_y - min_y);
    let area = side * side;

    writeln!(writer, "{}", area).expect("writeln! should work");
}
struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn bounding_box(rect1: &Rectangle, rect2: &Rectangle) -> (i32, i32, i32, i32) {
    let min_x = rect1.x1.min(rect2.x1);
    let min_y = rect1.y1.min(rect2.y1);
    let max_x = rect1.x2.max(rect2.x2);
    let max_y = rect1.y2.max(rect2.y2);
    (min_x, min_y, max_x, max_y)
}

// https://www.acmicpc.net/problem/14173
// Square Pasture
#[test]
fn test_solve14173() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6 6 8 8
1 8 4 9"
                .to_string(),
            want: "49".to_string(),
        },
        TestData {
            s: "0 0 10 10
5 5 6 6"
                .to_string(),
            want: "100".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14173(&mut reader, &mut writer);

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

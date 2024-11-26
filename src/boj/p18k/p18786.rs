use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18786(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let (x, y) = read_values_as!(read_line(reader), i32, i32);
        points.push(Point { x, y });
    }

    let max_area = find_max_area(&points);
    write!(writer, "{}", max_area).expect("Failed to write");
}

fn find_max_area(points: &[Point]) -> i32 {
    let n = points.len();
    let mut max_area = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (a, b, c) = (points[i], points[j], points[k]);
                if is_orthogonal_triangle(&a, &b, &c) {
                    max_area = max_area.max(calculate_area(&a, &b, &c));
                }
            }
        }
    }

    max_area
}

fn is_orthogonal_triangle(a: &Point, b: &Point, c: &Point) -> bool {
    (a.x == b.x && (b.y == c.y || c.y == a.y))
        || (b.x == c.x && (c.y == a.y || a.y == b.y))
        || (c.x == a.x && (a.y == b.y || b.y == c.y))
}

fn calculate_area(a: &Point, b: &Point, c: &Point) -> i32 {
    // NOTE - https://en.wikipedia.org/wiki/Shoelace_formula
    // 신발끈 공식
    ((a.x * b.y + b.x * c.y + c.x * a.y) - (a.y * b.x + b.y * c.x + c.y * a.x)).abs()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

// https://www.acmicpc.net/problem/18786
// Triangles (Bronze)
#[test]
fn test_solve18786() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
0 0
0 1
1 0
1 2"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "5
0 0
0 1
0 100
1 0
1 2"
            .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "4
0 0
0 100
1 0
1 2"
            .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "4
0 0
0 100
1 100
1 2"
            .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "4
0 0
1 0
1 100
1 2"
            .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "4
0 100
1 0
1 100
1 2"
            .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "4
0 0
1 0
0 100
1 2"
            .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "4
0 0
1 100
0 100
1 2"
            .to_string(),
            want: "100".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18786(&mut reader, &mut writer);

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

use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13221(reader: &mut impl BufRead, writer: &mut impl Write) {
    let start = read_point(reader);
    let n = read_value(read_line(reader));

    let mut closest = PointWithDist::new(read_point(reader), &start);

    for _ in 1..n {
        let curr = read_point(reader);
        closest.update_if_closer(&curr, &start);
    }

    write!(writer, "{} {}", closest.point.x, closest.point.y).expect("Failed to write");
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn read_point(reader: &mut impl BufRead) -> Point {
    let (x, y) = read_values_as!(read_line(reader), i32, i32);
    Point { x, y }
}

struct PointWithDist {
    point: Point,
    dist: i32,
}

impl PointWithDist {
    fn new(point: Point, other: &Point) -> Self {
        Self {
            point,
            dist: point.manhattan_distance(other),
        }
    }

    fn update_if_closer(&mut self, point: &Point, other: &Point) {
        let dist = point.manhattan_distance(other);
        if dist < self.dist {
            self.point = *point;
            self.dist = dist;
        }
    }
}

// https://www.acmicpc.net/problem/13221
// Manhattan
#[test]
fn test_solve13221() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 1
3
0 5
2 2
4 3"
            .to_string(),
            want: "2 2".to_string(),
        },
        TestData {
            s: "41 77
3
19 81
51 92
30 65"
                .to_string(),
            want: "30 65".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13221(&mut reader, &mut writer);

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

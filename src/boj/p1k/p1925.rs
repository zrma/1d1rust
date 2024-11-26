use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1925(reader: &mut impl BufRead, writer: &mut impl Write) {
    let a = read_point(reader);
    let b = read_point(reader);
    let c = read_point(reader);

    let ans = match (
        is_collinear(a, b, c),
        is_equilateral(a, b, c),
        is_isosceles(a, b, c),
    ) {
        (true, _, _) => "X",
        (_, true, _) => "JungTriangle",
        (_, _, true) => classify_isosceles(a, b, c),
        _ => classify_scalene(a, b, c),
    };

    write!(writer, "{}", ans).expect("write! should work");
}

fn classify_isosceles(a: Point, b: Point, c: Point) -> &'static str {
    match dot(a, b, c) {
        x if x < 0 => "Dunkak2Triangle",
        x if x > 0 => "Yeahkak2Triangle",
        _ => "Jikkak2Triangle",
    }
}

fn classify_scalene(a: Point, b: Point, c: Point) -> &'static str {
    match dot(a, b, c) {
        x if x < 0 => "DunkakTriangle",
        x if x > 0 => "YeahkakTriangle",
        _ => "JikkakTriangle",
    }
}

fn is_collinear(a: Point, b: Point, c: Point) -> bool {
    // 세 점이 일직선 상에 있는지 확인
    (a.x - b.x) * (a.y - c.y) == (a.x - c.x) * (a.y - b.y)
}

fn is_equilateral(a: Point, b: Point, c: Point) -> bool {
    let ab = a.distance_squared(&b);
    let bc = b.distance_squared(&c);
    let ca = c.distance_squared(&a);
    ab == bc && bc == ca
}

fn is_isosceles(a: Point, b: Point, c: Point) -> bool {
    let ab = a.distance_squared(&b);
    let bc = b.distance_squared(&c);
    let ca = c.distance_squared(&a);
    ab == bc || bc == ca || ca == ab
}

fn dot(a: Point, b: Point, c: Point) -> i32 {
    let ab = Vector { from: a, to: b };
    let ac = Vector { from: a, to: c };
    let ba = Vector { from: b, to: a };
    let bc = Vector { from: b, to: c };
    let ca = Vector { from: c, to: a };
    let cb = Vector { from: c, to: b };

    if ab.dot(&ac) < 0 || ba.dot(&bc) < 0 || ca.dot(&cb) < 0 {
        -1
    } else if ab.dot(&ac) == 0 || ba.dot(&bc) == 0 || ca.dot(&cb) == 0 {
        0
    } else {
        1
    }
}

struct Vector {
    from: Point,
    to: Point,
}

impl Vector {
    fn dot(&self, other: &Vector) -> i32 {
        (self.to.x - self.from.x) * (other.to.x - other.from.x)
            + (self.to.y - self.from.y) * (other.to.y - other.from.y)
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

fn read_point(reader: &mut impl BufRead) -> Point {
    let (x, y) = read_values_as!(read_line(reader), i32, i32);
    Point { x, y }
}

// https://www.acmicpc.net/problem/1925
// 삼각형
#[test]
fn test_solve1925() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0 0
-3 -3
-3 0"
                .to_string(),
            want: "Jikkak2Triangle".to_string(),
        },
        TestData {
            s: "1 1
0 0
2 100"
                .to_string(),
            want: "DunkakTriangle".to_string(),
        },
        TestData {
            s: "0 0
-1 -2
1 -2"
                .to_string(),
            want: "Yeahkak2Triangle".to_string(),
        },
        TestData {
            s: "10000 10000
2580 2580
-10000 -10000"
                .to_string(),
            want: "X".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1925(&mut reader, &mut writer);

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

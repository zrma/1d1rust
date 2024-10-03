use crate::read_values_as;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1894(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    let mut answers = Vec::new();
    while reader.read_line(&mut line).unwrap_or(0) > 0 {
        let (x1, y1, x2, y2, x3, y3, x4, y4) =
            read_values_as!(&line, f64, f64, f64, f64, f64, f64, f64, f64);

        let p1 = Point { x: x1, y: y1 };
        let p2 = Point { x: x2, y: y2 };
        let p3 = Point { x: x3, y: y3 };
        let p4 = Point { x: x4, y: y4 };

        let ans = find_point(p1, p2, p3, p4);
        answers.push(format!("{:.3} {:.3}", ans.x, ans.y));
        line.clear();
    }

    write!(writer, "{}", answers.join("\n")).expect("write! should work");
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn find_point(p1: Point, p2: Point, p3: Point, p4: Point) -> Point {
    fn calculate_new_point(p1: Point, p2: Point, p3: Point) -> Point {
        p3.add(p2).sub(p1)
    }

    if p1 == p3 {
        calculate_new_point(p1, p2, p4)
    } else if p1 == p4 {
        calculate_new_point(p1, p2, p3)
    } else if p2 == p3 {
        calculate_new_point(p2, p1, p4)
    } else if p2 == p4 {
        calculate_new_point(p2, p1, p3)
    } else {
        panic!("invalid input")
    }
}

// https://www.acmicpc.net/problem/1894
// 4번째 점
#[test]
fn test_solve1894() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0.000 0.000 0.000 1.000 0.000 1.000 1.000 1.000
1.000 0.000 3.500 3.500 3.500 3.500 0.000 1.000
1.866 0.000 3.127 3.543 3.127 3.543 1.412 3.145"
                .to_string(),
            want: "1.000 0.000
-2.500 -2.500
0.151 -0.398"
                .to_string(),
        },
        TestData {
            s: "0.000 0.000 0.000 1.000 0.000 0.000 1.000 0.000".to_string(),
            want: "1.000 1.000".to_string(),
        },
        TestData {
            s: "1.000 0.000 3.500 3.500 1.000 0.000 -2.500 -2.500
1.866 0.000 3.127 3.543 1.866 0.000 0.151 -0.398"
                .to_string(),
            want: "0.000 1.000
1.412 3.145"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1894(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

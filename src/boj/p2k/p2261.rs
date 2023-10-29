use std::cmp::Ordering;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2261(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let mut points = Vec::new();
    for _ in 0..n {
        s.clear();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        points.push(Point::new(x, y));
    }

    points.sort_by(|a, b| a.x.cmp(&b.x));

    let mut set = std::collections::BTreeSet::new();
    set.insert(points[0].clone());
    set.insert(points[1].clone());

    let mut ans = points[0].dist_pow2(&points[1]);
    let mut start = 0;

    for i in 2..n {
        let p = &points[i];
        while start < i {
            let q = &points[start];
            if p.x_diff_pow2(q) > ans {
                set.remove(q);
                start += 1;
            } else {
                break;
            }
        }

        let d = (ans as f64).sqrt().ceil() as i32;
        let lower = Point::new(-10000, p.y - d);
        let upper = Point::new(10000, p.y + d);

        for q in set.range(lower..=upper) {
            ans = ans.min(p.dist_pow2(q));
        }

        set.insert(p.clone());
    }

    write!(writer, "{}", ans).unwrap();
}

#[derive(Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist_pow2(&self, other: &Self) -> i32 {
        self.x_diff_pow2(other) + self.y_diff_pow2(other)
    }

    fn x_diff_pow2(&self, other: &Self) -> i32 {
        (self.x - other.x) * (self.x - other.x)
    }

    fn y_diff_pow2(&self, other: &Self) -> i32 {
        (self.y - other.y) * (self.y - other.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// https://www.acmicpc.net/problem/2261
// 가장 가까운 두 점
#[test]
fn test_solve2261() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
0 0
10 10
0 10
10 0"
                .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "5
0 0
10 10
0 10
10 0
5 0"
            .to_string(),
            want: "25".to_string(),
        },
        TestData {
            s: "4
0 0
10000 10000
-10000 -10000
0 0"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "4
0 0
10000 10000
-10000 -10000
1 1"
            .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2261(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

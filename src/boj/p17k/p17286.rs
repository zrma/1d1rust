use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17286(reader: &mut impl BufRead, writer: &mut impl Write) {
    let start = read_point(reader);
    let points = (0..3).map(|_| read_point(reader)).collect::<Vec<_>>();

    let mut min_dist = f64::MAX;
    permute(&points, 0, &mut |p| {
        let mut tot_dist = start.distance_to(&p[0]);
        tot_dist += p.windows(2).map(|w| w[0].distance_to(&w[1])).sum::<f64>();

        min_dist = min_dist.min(tot_dist);
    });

    write!(writer, "{}", min_dist as i32).expect("Failed to write");
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn read_point(reader: &mut impl BufRead) -> Point {
    let (x, y) = read_values!(read_line(reader), f64, f64);
    Point { x, y }
}

fn permute<F: FnMut(&[Point])>(points: &[Point], start: usize, f: &mut F) {
    if start == points.len() {
        f(points);
    } else {
        let mut points = points.to_vec(); // points의 복사본을 만들어서 사용.
        for i in start..points.len() {
            points.swap(start, i);
            permute(&points, start + 1, f); // 복사본에 대해 재귀 호출
        }
    }
}

// https://www.acmicpc.net/problem/17286
// 유미
#[test]
fn test_solve17286() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0 0
1 0
2 0
4 0"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "0 0
-1 -1
1 -1
1 1"
            .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17286(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

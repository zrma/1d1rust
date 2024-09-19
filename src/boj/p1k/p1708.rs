use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1708(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let (x, y) = read_values_as!(read_line(reader), i64, i64);
        points.push(Point { x, y });
    }

    let ans = convex_hull(&mut points);
    write!(writer, "{}", ans).expect("Failed to write");
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

fn convex_hull(points: &mut [Point]) -> usize {
    // 점들을 x좌표 기준으로 정렬하고, 같다면 y좌표 기준으로 정렬
    points.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    let mut hull = Vec::new();

    // 아래 껍질 구성
    for &point in points.iter() {
        while hull.len() >= 2 && ccw(hull[hull.len() - 2], *hull.last().unwrap(), point) <= 0 {
            hull.pop();
        }
        hull.push(point);
    }

    // 위 껍질 구성
    let lower_len = hull.len();
    for &point in points.iter().rev() {
        while hull.len() > lower_len && ccw(hull[hull.len() - 2], *hull.last().unwrap(), point) <= 0
        {
            hull.pop();
        }
        hull.push(point);
    }

    // 시작점이 중복되므로 제거
    hull.pop();

    // 볼록 껍질을 이루는 점의 개수 반환
    hull.len()
}

fn ccw(p1: Point, p2: Point, p3: Point) -> i64 {
    // 세 점의 방향성을 계산 (반시계 방향이면 양수, 시계 방향이면 음수, 일직선이면 0)
    (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
}

// https://www.acmicpc.net/problem/1708
// 볼록 껍질
#[test]
fn test_solve1708() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8
1 1
1 2
1 3
2 1
2 2
2 3
3 1
3 2"
            .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "9
1 1
1 2
1 3
2 1
2 2
2 3
3 1
3 2
3 3"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "25
-2 -2
-2 -1
-2 0
-2 1
-2 2
-1 -2
-1 -1
-1 0
-1 1
-1 2
0 -2
0 -1
0 0
0 1
0 2
1 -2
1 -1
1 0
1 1
1 2
2 -2
2 -1
2 0
2 1
2 2"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "7
6 5
2 3
7 4
0 3
3 3
3 8
8 4"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "8
0 0
1 0
2 0
2 1
2 2
1 2
0 2
0 1"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "10
1 1
3 2
5 3
7 4
9 5
1 9
-1 2
-3 3
-5 4
-7 5"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "19
0 0
0 1
0 2
0 3
0 4
0 5
0 6
0 7
0 8
0 9
1 0
2 0
3 0
4 0
5 0
6 0
7 0
8 0
9 0"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "17
0 0
0 1
0 2
0 3
0 4
0 5
0 6
0 7
0 8
1 0
2 0
3 0
4 0
5 0
6 0
7 0
8 0"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5
40000 40000
-40000 -40000
40000 -40000
0 0
-40000 40000"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "9
40000 40000
0 0
-40000 0
40000 0
0 40000
-40000 40000
40000 -40000
-40000 -40000
0 -40000"
                .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1708(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

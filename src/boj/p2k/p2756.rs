use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2756(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    let mut answers = Vec::with_capacity(t);
    for _ in 0..t {
        let points = read_points(read_line(reader));
        let scores = points.iter().map(calc_score).collect::<Vec<_>>();
        let (score1, score2) = scores.split_at(3);

        let ans = compare_scores(score1.iter().sum(), score2.iter().sum());
        answers.push(ans);
    }

    writeln!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

fn read_points(s: String) -> Vec<Point> {
    s.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>() // Collect all numbers first
        .chunks(2) // Split them into chunks of two
        .map(|chunk| Point {
            x: chunk[0],
            y: chunk[1],
        }) // Create points from the chunks
        .collect()
}

fn compare_scores(score1: i32, score2: i32) -> String {
    match score1.cmp(&score2) {
        Ordering::Greater => format!("SCORE: {} to {}, PLAYER 1 WINS.", score1, score2),
        Ordering::Less => format!("SCORE: {} to {}, PLAYER 2 WINS.", score1, score2),
        Ordering::Equal => format!("SCORE: {} to {}, TIE.", score1, score2),
    }
}

fn calc_score(point: &Point) -> i32 {
    let dist = point.x.hypot(point.y);
    match dist {
        d if d <= 3.0 => 100,
        d if d <= 6.0 => 80,
        d if d <= 9.0 => 60,
        d if d <= 12.0 => 40,
        d if d <= 15.0 => 20,
        _ => 0,
    }
}

struct Point {
    x: f64,
    y: f64,
}

// https://www.acmicpc.net/problem/2756
// 다트
#[test]
fn test_solve2756() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
-9 0 0 -4.5 -2 2 9 0 0 4.5 2 -2
-19.0 19.0 0 0 0 0 3 3 6 6 12 12"
                .to_string(),
            want: "SCORE: 240 to 240, TIE.
SCORE: 200 to 140, PLAYER 1 WINS.
"
            .to_string(),
        },
        TestData {
            s: "1
3 3 6 6 12 12 -19.0 19.0 0 0 0 0"
                .to_string(),
            want: "SCORE: 140 to 200, PLAYER 2 WINS.
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2756(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

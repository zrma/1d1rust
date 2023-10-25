use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9772(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let (x, y) = {
            let s = read_line(reader);
            let mut iter = s.split_whitespace();
            (
                iter.next().unwrap().parse::<f64>().unwrap(),
                iter.next().unwrap().parse::<f64>().unwrap(),
            )
        };

        let ans = get_quadrants(x, y);
        writeln!(writer, "{}", ans).unwrap();

        if x == 0.0 && y == 0.0 {
            break;
        }
    }
}

fn get_quadrants(x: f64, y: f64) -> String {
    if x == 0.0 || y == 0.0 {
        "AXIS".to_string()
    } else if x > 0.0 && y > 0.0 {
        "Q1".to_string()
    } else if x < 0.0 && y > 0.0 {
        "Q2".to_string()
    } else if x < 0.0 && y < 0.0 {
        "Q3".to_string()
    } else {
        "Q4".to_string()
    }
}

// https://www.acmicpc.net/problem/9772
// Quadrants
#[test]
fn test_solve9772() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2
-1 -0.12
4 0
-10.4 200
0 0"
            .to_string(),
            want: "Q1
Q3
AXIS
Q2
AXIS
"
            .to_string(),
        },
        TestData {
            s: "1 2
0 0"
            .to_string(),
            want: "Q1
AXIS
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9772(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2985(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c) = read_values_as!(read_line(reader), i32, i32, i32);

    if let Some(expression) = find_expression(a, b, c) {
        write!(writer, "{}", expression).unwrap();
    }
}

fn find_expression(a: i32, b: i32, c: i32) -> Option<String> {
    let ops = ['+', '-', '*', '/'];

    for &op in &ops {
        if let Some(result) = apply_op(op, a, b) {
            if result == c {
                return Some(format!("{}{}{}={}", a, op, b, c));
            }
        }

        if let Some(result) = apply_op(op, b, c) {
            if result == a {
                return Some(format!("{}={}{}{}", a, b, op, c));
            }
        }
    }

    None
}

fn apply_op(op: char, x: i32, y: i32) -> Option<i32> {
    match op {
        '+' => Some(x + y),
        '-' => Some(x - y),
        '*' => Some(x * y),
        '/' => {
            if y == 0 || x % y != 0 {
                None
            } else {
                Some(x / y)
            }
        }
        _ => None,
    }
}

// https://www.acmicpc.net/problem/2985
// 세 수
#[test]
fn test_solve2985() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 3 8".to_string(),
            want: "5+3=8".to_string(),
        },
        TestData {
            s: "5 15 3".to_string(),
            want: "5=15/3".to_string(),
        },
        TestData {
            s: "8 5 3".to_string(),
            want: "8=5+3".to_string(),
        },
        TestData {
            s: "8 3 5".to_string(),
            want: "8=3+5".to_string(),
        },
        TestData {
            s: "5 3 15".to_string(),
            want: "5*3=15".to_string(),
        },
        TestData {
            s: "15 3 5".to_string(),
            want: "15=3*5".to_string(),
        },
        TestData {
            s: "15 5 3".to_string(),
            want: "15=5*3".to_string(),
        },
        TestData {
            s: "1 2 2".to_string(),
            want: "1*2=2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2985(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

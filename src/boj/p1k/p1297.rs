use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1297(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut iter = s.split_whitespace();
    let d = iter.next().unwrap().parse::<f64>().unwrap();
    let h = iter.next().unwrap().parse::<f64>().unwrap();
    let w = iter.next().unwrap().parse::<f64>().unwrap();

    let d2 = d * d;
    let h2 = h * h;
    let w2 = w * w;

    let x = (d2 / (h2 + w2)).sqrt();
    let y = x * h;
    let z = x * w;

    write!(writer, "{} {}", y as i64, z as i64).unwrap();
}

// https://www.acmicpc.net/problem/1297
// TV 크기
// noinspection SpellCheckingInspection
#[test]
fn test_solve1297() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "52 9 16".to_string(),
            want: "25 45".to_string(),
        },
        TestData {
            s: "7 2 3".to_string(),
            want: "3 5".to_string(),
        },
        TestData {
            s: "5 3 4".to_string(),
            want: "3 4".to_string(),
        },
        TestData {
            s: "13 5 12".to_string(),
            want: "5 12".to_string(),
        },
        TestData {
            s: "13 7 10".to_string(),
            want: "7 10".to_string(),
        },
        TestData {
            s: "7 32 47".to_string(),
            want: "3 5".to_string(),
        },
        TestData {
            s: "11 15 16".to_string(),
            want: "7 8".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1297(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

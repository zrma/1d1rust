use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2217(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let mut ropes: Vec<i32> = (0..n)
        .map(|_| read_line(reader).trim().parse::<i32>().unwrap())
        .collect();

    ropes.sort_unstable_by(|a, b| b.cmp(a));

    let max_weight = ropes
        .iter()
        .enumerate()
        .map(|(i, &rope)| rope * (i as i32 + 1))
        .max()
        .unwrap_or(0);

    write!(writer, "{}", max_weight).unwrap();
}

// https://www.acmicpc.net/problem/2217
// 로프
#[test]
fn test_solve2217() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
10
15"
            .to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "3
10
15
5"
            .to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "3
10
15
7"
            .to_string(),
            want: "21".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2217(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

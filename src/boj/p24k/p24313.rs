use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24313(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a1, a0) = {
        let line = read_line(reader);
        let mut s = line.split_whitespace();
        let a1 = s.next().unwrap().parse::<i64>().unwrap();
        let a0 = s.next().unwrap().parse::<i64>().unwrap();
        (a1, a0)
    };

    let c = read_line(reader).parse::<i64>().unwrap();
    let n0 = read_line(reader).parse::<i64>().unwrap();

    // f(n) = a1 * n + a0
    // g(n) ~= n
    // f(n) <= c * g(n)
    // a1 * n + a0 <= c * n
    // a0 <= (c - a1) * n

    if (c - a1) >= 0 && (c - a1) * n0 >= a0 {
        write!(writer, "1").unwrap();
        return;
    }

    write!(writer, "0").unwrap()
}

// https://www.acmicpc.net/problem/24313
// 알고리즘 수업 - 점근적 표기 1
#[test]
fn test_solve24313() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7 7
8
1"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "7 7
8
10"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve24313(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

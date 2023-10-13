use crate::utils::io::read_line;
use num::integer::gcd;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2485(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let mut v = vec![];
    for _ in 0..n {
        v.push(read_line(reader).parse::<i64>().unwrap());
    }

    let mut gcd_val = v[1] - v[0];
    for i in 2..n {
        gcd_val = gcd(gcd_val, v[i] - v[i - 1]);
    }

    let mut ans = 0;
    for i in 1..n {
        ans += (v[i] - v[i - 1]) / gcd_val - 1;
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2485
// 가로수
#[test]
fn test_solve2485() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
1
3
7
13"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "4
2
6
12
18"
            .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2485(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

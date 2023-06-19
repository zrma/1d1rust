use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3449(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<i64>().unwrap();

    for _ in 0..t {
        let s1 = read_line(reader);
        let s2 = read_line(reader);

        let mut diff = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                diff += 1;
            }
        }

        writeln!(writer, "Hamming distance is {}.", diff).unwrap();
    }
}

// https://www.acmicpc.net/problem/3449
// 해밍 거리
#[test]
fn test_solve3449() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
        s: "4
0
1
000
000
1111111100000000
0000000011111111
101
000"
        .to_string(),
        want: "Hamming distance is 1.
Hamming distance is 0.
Hamming distance is 16.
Hamming distance is 2.
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3449(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

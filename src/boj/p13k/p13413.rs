use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13413(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();
    for _ in 0..t {
        let _ = read_line(reader).parse::<usize>().unwrap();
        let from = read_line(reader);
        let to = read_line(reader);

        let mut w = 0;
        let mut b = 0;
        for (f, t) in from.chars().zip(to.chars()) {
            if f != t {
                if f == 'W' {
                    w += 1;
                } else {
                    b += 1;
                }
            }
        }

        let ans = if w > b { w } else { b };
        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/13413
// 오셀로 재배치
// noinspection SpellCheckingInspection
#[test]
fn test_solve13413() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
        s: "3
5
WBBWW
WBWBW
7
BBBBBBB
BWBWBWB
4
WWBB
BBWB"
            .to_string(),
        want: "1
3
2
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13413(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2490(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut ans = Vec::with_capacity(3);

    for _ in 0..3 {
        let input = read_yut_line(reader);
        ans.push(match input {
            0 => "D",
            1 => "C",
            2 => "B",
            3 => "A",
            4 => "E",
            _ => unreachable!("invalid input"),
        });
    }

    write!(writer, "{}", ans.join("\n")).unwrap();
}

fn read_yut_line(reader: &mut impl BufRead) -> usize {
    read_line(reader)
        .split_whitespace()
        .take(4)
        .map(|s| s.parse::<usize>().unwrap())
        .sum()
}

// https://www.acmicpc.net/problem/2490
// 윷놀이
#[test]
fn test_solve2490() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0 1 0 1
1 1 1 0
0 0 1 1"
                .to_string(),
            want: "B
A
B"
            .to_string(),
        },
        TestData {
            s: "0 0 0 1
0 0 0 0
1 1 1 1"
                .to_string(),
            want: "C
D
E"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2490(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1672(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let s = read_line(reader);

    let mut dna = vec![];
    for c in s.chars().take(n) {
        dna.push(c);
    }

    let mut ans = dna.pop().unwrap();
    while let Some(c) = dna.pop() {
        ans = match (ans, c) {
            ('A', 'A') => 'A',
            ('A', 'G') => 'C',
            ('A', 'C') => 'A',
            ('A', 'T') => 'G',
            ('G', 'A') => 'C',
            ('G', 'G') => 'G',
            ('G', 'C') => 'T',
            ('G', 'T') => 'A',
            ('C', 'A') => 'A',
            ('C', 'G') => 'T',
            ('C', 'C') => 'C',
            ('C', 'T') => 'G',
            ('T', 'A') => 'G',
            ('T', 'G') => 'A',
            ('T', 'C') => 'G',
            ('T', 'T') => 'T',
            _ => unreachable!(),
        };
    }
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/1672
// DNA 해독
// noinspection SpellCheckingInspection
#[test]
fn test_solve1672() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6
AAGTCG"
                .to_string(),
            want: "A".to_string(),
        },
        TestData {
            s: "4
AAGT"
                .to_string(),
            want: "A".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1672(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

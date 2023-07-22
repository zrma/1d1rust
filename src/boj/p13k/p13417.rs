use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13417(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();
    for _ in 0..t {
        let n = read_line(reader).parse::<usize>().unwrap();
        let line = read_line(reader);
        let cards = line.splitn(n, ' ').collect::<Vec<_>>();

        let mut ans = String::new();
        ans.push_str(cards[0]);

        cards.iter().skip(1).for_each(|&card| {
            if card <= &ans[0..1] {
                ans.insert_str(0, card);
            } else {
                ans.push_str(card);
            }
        });
        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/13417
// 카드 문자열
// noinspection SpellCheckingInspection
#[test]
fn test_solve13417() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
        s: "3
3
M K U
5
A S D F G
7
B A C A B A C"
            .to_string(),
        want: "KMU
ASDFG
AAABCBC
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13417(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
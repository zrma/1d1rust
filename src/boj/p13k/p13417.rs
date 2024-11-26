use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13417(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    for _ in 0..t {
        let n = read_value(read_line(reader));
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
        writeln!(writer, "{}", ans).expect("Failed to write");
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
    for (i, data) in [TestData {
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

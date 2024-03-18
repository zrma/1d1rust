use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11507(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut cards = vec![vec![0; 14]; 4];
    let suits = ['P', 'K', 'H', 'T'];

    for i in 0..s.len() / 3 {
        let suit_char = s.chars().nth(i * 3).unwrap();
        let suit_index = suits.iter().position(|&x| x == suit_char).unwrap();
        let num = s[i * 3 + 1..i * 3 + 3].parse::<usize>().unwrap();
        if cards[suit_index][num] == 1 {
            write!(writer, "GRESKA").expect("Failed to write");
            return;
        }
        cards[suit_index][num] = 1;
    }

    let ans = cards
        .iter()
        .map(|card| (13 - card.iter().sum::<usize>()).to_string())
        .collect::<Vec<String>>();

    write!(writer, "{}", ans.join(" ")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/11507
// noinspection SpellCheckingInspection
// 카드셋트
#[test]
fn test_solve11507() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "P01K02H03H04".to_string(),
            want: "12 12 11 13".to_string(),
        },
        TestData {
            s: "H02H10P11H02".to_string(),
            want: "GRESKA".to_string(),
        },
        TestData {
            s: "P10K10H10T01".to_string(),
            want: "12 12 12 12".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11507(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

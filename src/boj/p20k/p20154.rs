use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20154(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let s = s.as_bytes();

    let values = [
        3, 2, 1, 2, 3, 3, 3, 3, 1, 1, 3, 1, 3, 3, 1, 2, 2, 2, 1, 2, 1, 1, 2, 2, 2, 1,
    ];

    let sum = s
        .iter()
        .map(|&c| values[(c - b'A') as usize] as u32)
        .sum::<u32>()
        % 10;

    let ans = if sum % 2 == 0 {
        "You're the winner?"
    } else {
        "I'm a winner!"
    };
    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/20154
// noinspection SpellCheckingInspection
// 이 구역의 승자는 누구야?!
#[test]
fn test_solve20154() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ABCDE".to_string(),
            want: "I'm a winner!".to_string(),
        },
        TestData {
            s: "AECF".to_string(),
            want: "You're the winner?".to_string(),
        },
        TestData {
            s: "A".to_string(),
            want: "I'm a winner!".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve20154(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

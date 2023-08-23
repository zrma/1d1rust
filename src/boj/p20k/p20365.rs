use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20365(reader: &mut impl BufRead, writer: &mut impl Write) {
    read_line(reader);
    let chars = read_line(reader).chars().collect::<Vec<_>>();

    let mut r_count = 0;
    let mut b_count = 0;
    let mut prev = ' ';

    chars.iter().for_each(|&ch| {
        if ch != prev {
            if ch == 'R' {
                r_count += 1;
            } else {
                b_count += 1;
            }
        }
        prev = ch;
    });

    let ans = r_count.min(b_count) + 1;
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/20365
// 블로그2
// noinspection SpellCheckingInspection
#[test]
fn test_solve20365() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "8
BBRBRBBR"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "8
BRBRRBRB"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "8
BRBBBBRB"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "8
BBBBBBBB"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "8
BBBBBBBR"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "8
BRRRRRRR"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "8
BRRRRRRB"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20365(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}

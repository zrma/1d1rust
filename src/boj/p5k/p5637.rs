use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5637(reader: &mut impl BufRead, writer: &mut impl Write) {
    let is_word = |c: char| c.is_alphabetic() || c == '-';

    let mut longest_word = String::new();

    'outer: loop {
        let line = read_line(reader);
        let words = line.split(|c: char| !is_word(c));

        for word in words {
            if word == "E-N-D" {
                break 'outer;
            }
            if word.len() > longest_word.len() {
                longest_word = word.to_lowercase();
            }
        }
    }

    write!(writer, "{}", longest_word).expect("Failed to write");
}

// https://www.acmicpc.net/problem/5637
// noinspection SpellCheckingInspection
// 가장 긴 단어
#[test]
fn test_solve5637() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "  ACM International Collegiate Programming Contest (abbreviated as
ACM-ICPC or just ICPC) is an annual multi-tiered computer programming
competition among the universities of the world. The contest is
sponsored by IBM. Headquartered at Baylor University, with autonomous
regions on six continents, the ICPC is directed by Baylor Professor
William B. Poucher, Executive Director, and operates under the
auspices of the Association for Computing Machinery (ACM).

  The 2012 ACM-ICPC Asia Hatyai Regional Programming Contest is
held during 15-16 November 2012. It is hosted by Prince of Songkla
University, Hatyai campus. E-N-D"
                .to_string(),
            want: "international".to_string(),
        },
        TestData {
            s: "Pen pineapple apple pen, Pen nineapple apple pen. E-N-D
PenPineappleApplePen"
                .to_string(),
            want: "pineapple".to_string(),
        },
        TestData {
            s: "aa, bbb E-N-D".to_string(),
            want: "bbb".to_string(),
        },
        TestData {
            s: "abcd.efgh E-N-D".to_string(),
            want: "abcd".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve5637(&mut reader, &mut writer);

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

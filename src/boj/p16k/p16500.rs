use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16500(reader: &mut impl BufRead, writer: &mut impl Write) {
    let target = read_line(reader);
    let num_words = read_value(read_line(reader));
    let words = (0..num_words)
        .map(|_| read_line(reader))
        .collect::<Vec<_>>();

    let mut can_form = vec![false; target.len() + 1];
    can_form[0] = true;

    for end in 1..=target.len() {
        for word in &words {
            let start = end.saturating_sub(word.len());
            if can_form[start] && &target[start..end] == word {
                can_form[end] = true;
                break;
            }
        }
    }

    let ans = if can_form[target.len()] { 1 } else { 0 };
    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/16500
// noinspection SpellCheckingInspection
// 문자열 판별
#[test]
fn test_solve16500() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "softwarecontest
2
software
contest"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "softwarecontestx
2
software
contest"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "applepiechip
3
apple
pie
chip"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "applepiechip
2
apple
pie"
            .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16500(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

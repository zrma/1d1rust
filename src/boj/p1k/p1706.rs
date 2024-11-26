use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1706(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    let mut board = Vec::with_capacity(n);
    for _ in 0..n {
        let s = read_line(reader).chars().take(m).collect::<Vec<_>>();
        board.push(s);
    }

    let mut answer: Option<String> = None;
    for line in &board {
        let v: String = line.iter().collect::<_>();
        v.split('#')
            .filter(|&s| s.len() > 1)
            .for_each(|word| update_answer(&mut answer, word));
    }

    for col in 0..m {
        let word: String = board.iter().map(|line| line[col]).collect();
        word.split('#')
            .filter(|&s| s.len() > 1)
            .for_each(|word| update_answer(&mut answer, word));
    }

    write!(writer, "{}", answer.unwrap_or_default()).expect("write! should work");
}

fn update_answer(answer: &mut Option<String>, word: &str) {
    match answer.as_deref() {
        Some(ans) if word < ans => *answer = Some(word.to_string()),
        None => *answer = Some(word.to_string()),
        _ => {}
    }
}

// https://www.acmicpc.net/problem/1706
// noinspection SpellCheckingInspection
// 크로스워드
#[test]
fn test_solve1706() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 5
adaca
da##b
abb#b
abbac"
                .to_string(),
            want: "abb".to_string(),
        },
        TestData {
            s: "5 5
good#
an##b
messy
e##it
#late"
                .to_string(),
            want: "an".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve1706(&mut reader, &mut writer);

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

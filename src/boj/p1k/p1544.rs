use crate::utils::io::{read_line, read_value};
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1544(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut words = HashSet::with_capacity(n);
    let mut ans = 0;

    for _ in 0..n {
        let word = read_line(reader);

        if !words.contains(&word) {
            insert_rotated_words(&word, &mut words);
            ans += 1;
        }
    }

    writeln!(writer, "{}", ans).unwrap();
}

fn insert_rotated_words(word: &str, words: &mut HashSet<String>) {
    for i in 0..word.len() {
        let rotated = rotate(word, i);
        words.insert(rotated);
    }
}

fn rotate(s: &str, n: usize) -> String {
    let mut rotated = String::with_capacity(s.len());
    rotated.extend(s.chars().skip(n));
    rotated.extend(s.chars().take(n));
    rotated
}

// https://www.acmicpc.net/problem/1544
// noinspection SpellCheckingInspection
// 사이클 단어
#[test]
fn test_solve1544() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
picture
turepic
icturep
word
ordw"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "7
ast
ats
tas
tsa
sat
sta
ttt"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5
aaaa
aaa
aa
aaaa
aaaaa"
                .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1544(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

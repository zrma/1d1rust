use crate::utils::io::{read_line, read_value};
use std::collections::HashMap;
use std::io::{BufRead, Write};

// noinspection SpellCheckingInspection
#[allow(dead_code)]
fn solve1411(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_words: usize = read_value(read_line(reader));
    let mut unique_words = HashMap::new();

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    for _ in 0..num_of_words {
        let word = read_line(reader);
        let mut word_map = HashMap::new();
        let mut word_index = 0;
        let mut new_word = String::new();

        for c in word.chars() {
            if let Some(&index) = word_map.get(&c) {
                new_word.push(alphabet[index]);
            } else {
                word_map.insert(c, word_index);
                new_word.push(alphabet[word_index]);
                word_index += 1;
            }
        }

        *unique_words.entry(new_word).or_insert(0) += 1;
    }

    let ans: usize = unique_words.iter().map(|(_, &v)| v * (v - 1) / 2).sum();

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/1411
// noinspection SpellCheckingInspection
// 비슷한 단어
#[test]
fn test_solve1411() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
aa
ab
bb
cc
cd"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "3
abca
zbxz
opqr"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "12
cacccdaabc
cdcccaddbc
dcdddbccad
bdbbbaddcb
bdbcadbbdc
abaadcbbda
babcdabbac
cacdbaccad
dcddabccad
cacccbaadb
bbcdcbcbdd
bcbadcbbca"
                .to_string(),
            want: "13".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve1411(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

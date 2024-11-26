use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9081(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_value(read_line(reader));
    let mut answers: Vec<String> = Vec::with_capacity(t);

    for _ in 0..t {
        let mut word = read_line(reader).chars().collect::<Vec<char>>();
        process_word(&mut word);
        answers.push(word.into_iter().collect());
    }

    write!(writer, "{}", answers.join("\n")).unwrap();
}

fn process_word(word: &mut [char]) {
    if let Some(pivot) = find_decreasing_point(word) {
        let swap_index = word.iter().rposition(|&c| c > word[pivot - 1]).unwrap();
        word.swap(pivot - 1, swap_index);
        word[pivot..].reverse();
    }
}

fn find_decreasing_point(word: &[char]) -> Option<usize> {
    (1..word.len()).rev().find(|&i| word[i - 1] < word[i])
}

// https://www.acmicpc.net/problem/9081
// noinspection SpellCheckingInspection
// 단어 맞추기
#[test]
fn test_solve9081() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
HELLO
DRINK
SHUTTLE
ZOO"
            .to_string(),
            want: "HELOL
DRKIN
SLEHTTU
ZOO"
            .to_string(),
        },
        TestData {
            s: "12
BEER
BERE
BREE
EBER
EBRE
EEBR
EERB
ERBE
EREB
RBEE
REBE
REEB"
                .to_string(),
            want: "BERE
BREE
EBER
EBRE
EEBR
EERB
ERBE
EREB
RBEE
REBE
REEB
REEB"
                .to_string(),
        },
        TestData {
            s: "2
bcade
badec"
                .to_string(),
            want: "bcaed
baecd"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9081(&mut reader, &mut writer);

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

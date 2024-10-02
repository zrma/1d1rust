use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2954(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let res = replace_vowels(line);

    write!(writer, "{}", res).expect("Failed to write");
}

fn replace_vowels(input: String) -> String {
    let mut res = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        res.push(c);

        if is_vowel(c) {
            chars.next();
            chars.next();
        }
    }

    res
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

// https://www.acmicpc.net/problem/2954
// 창영이의 일기장
// noinspection SpellCheckingInspection
#[test]
fn test_solve2954() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "zepelepenapa papapripikapa".to_string(),
            want: "zelena paprika".to_string(),
        },
        TestData {
            s: "bapas jepe doposapadnapa opovapa kepemipijapa ".to_string(),
            want: "bas je dosadna ova kemija".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2954(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

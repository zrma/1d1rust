use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11319(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let ans = (0..num_cases)
        .map(|_| {
            let line = read_line(reader);
            count_vowels_and_consonants(&line)
        })
        .map(|(vowels, consonants)| format!("{} {}", consonants, vowels))
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).unwrap();
}

fn count_vowels_and_consonants(line: &str) -> (usize, usize) {
    line.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold((0, 0), |(vowels, consonants), c| {
            if is_vowel(c) {
                (vowels + 1, consonants)
            } else {
                (vowels, consonants + 1)
            }
        })
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

// https://www.acmicpc.net/problem/11319
// noinspection SpellCheckingInspection
// Count Me In
#[test]
fn test_solve11319() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
You can win this thing
May be too optimistic
Just try to have fun"
                .to_string(),
            want: "12 6
10 8
11 5"
                .to_string(),
        },
        TestData {
            s: "1
I am happy"
                .to_string(),
            want: "5 3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11319(&mut reader, &mut writer);

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

use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
// noinspection SpellCheckingInspection
fn solve10384(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let mut answers = Vec::with_capacity(num_cases);
    for case_num in 1..=num_cases {
        let sentence = read_line(reader);
        let mut letter_counts = [0; 26];

        for char in sentence.chars().map(|c| c.to_ascii_lowercase()) {
            let index = match char {
                'a'..='z' => (char as u8 - b'a') as usize,
                _ => continue,
            };
            letter_counts[index] += 1;
        }

        let min_count = *letter_counts.iter().min().unwrap();
        let verdict = match min_count {
            0 => "Not a pangram",
            1 => "Pangram!",
            2 => "Double pangram!!",
            _ => "Triple pangram!!!",
        };

        answers.push(format!("Case {}: {}", case_num, verdict));
    }

    write!(writer, "{}", answers.join("\n")).expect("write! should work");
}

// https://www.acmicpc.net/problem/10384
// noinspection SpellCheckingInspection
// 팬그램
#[test]
fn test_solve10384() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
The quick brown fox jumps over a lazy dog.
The quick brown fox jumps over a laconic dog.
abcdefghijklmNOPQRSTUVWXYZ-zyxwvutsrqpon   2013/2014      MLKJIHGFEDCBA"
                .to_string(),
            want: "Case 1: Pangram!
Case 2: Not a pangram
Case 3: Double pangram!!"
                .to_string(),
        },
        TestData {
            s: "1
Hello World"
                .to_string(),
            want: "Case 1: Not a pangram".to_string(),
        },
        TestData {
            s: "1
ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .to_string(),
            want: "Case 1: Triple pangram!!!".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10384(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

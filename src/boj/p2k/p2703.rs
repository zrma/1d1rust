use crate::utils::functions::char_to_index;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2703(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value(read_line(reader));
    const SPACE_INDEX: usize = 26;

    let mut decrypted_sentences = Vec::with_capacity(num_cases);
    for _ in 0..num_cases {
        let encrypted_sentence = read_line(reader);
        let decryption_key = read_line(reader).chars().enumerate().fold(
            [' '; SPACE_INDEX + 1],
            |mut acc, (index, char)| {
                acc[index] = char;
                acc
            },
        );

        let decrypted_sentence: String = encrypted_sentence
            .chars()
            .map(|char| match char {
                'A'..='Z' => decryption_key[char_to_index::<usize>(char)],
                _ => char,
            })
            .collect::<_>();

        decrypted_sentences.push(decrypted_sentence);
    }

    write!(writer, "{}", decrypted_sentences.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/2703
// noinspection SpellCheckingInspection
// Cryptoquote
#[test]
fn test_solve2703() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
HPC PJVYMIY
BLMRGJIASOPZEFDCKWYHUNXQTV
FDY GAI BG UKMY
KIMHOTSQYRLCUZPAGWJNBVDXEF"
                .to_string(),
            want: "ACM CONTEST
THE SKY IS BLUE"
                .to_string(),
        },
        TestData {
            s: "1
HELLO WORLD
ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .to_string(),
            want: "HELLO WORLD".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2703(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

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
        let key_line = read_line(reader);
        let mut decryption_key = [' '; SPACE_INDEX + 1];
        for (index, ch) in key_line.chars().enumerate() {
            decryption_key[index] = ch;
        }

        let decrypted_sentence: String = encrypted_sentence
            .chars()
            .map(|ch| match ch {
                'A'..='Z' => decryption_key[char_to_index::<usize>(ch)],
                _ => ch,
            })
            .collect();

        decrypted_sentences.push(decrypted_sentence);
    }

    writeln!(writer, "{}", decrypted_sentences.join("\n")).unwrap();
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

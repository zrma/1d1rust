use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9046(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value(read_line(reader));
    let mut answers = Vec::with_capacity(num_cases);
    for _ in 0..num_cases {
        let line = read_line(reader);
        let mut counts = [0; 26];
        for char in line.chars() {
            if char.is_ascii_alphabetic() {
                let index = char.to_ascii_lowercase() as usize - 'a' as usize;
                counts[index] += 1;
            }
        }

        let max_count = *counts.iter().max().unwrap();
        let max_indices: Vec<_> = counts
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count == max_count)
            .collect();

        let decrypted = if max_indices.len() == 1 {
            let (index, _) = max_indices[0];
            (index as u8 + b'a') as char
        } else {
            '?'
        };
        answers.push(decrypted.to_string());
    }
    writeln!(writer, "{}", answers.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/9046
// noinspection SpellCheckingInspection
// 복호화
#[test]
fn test_solve9046() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
asvdge ef ofmdofn
xvssc kxvbv
hull full suua pmlu"
                .to_string(),
            want: "f
v
?"
            .to_string(),
        },
        TestData {
            s: "2
aaaa bbbb cccc dddd
aaaa aaaa bbbb cccc"
                .to_string(),
            want: "?
a"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9046(&mut reader, &mut writer);

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

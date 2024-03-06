use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14713(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_sentences = read_value(read_line(reader));

    let mut sentences = (0..num_sentences)
        .map(|_| {
            read_line(reader)
                .split_whitespace()
                .map(|word| word.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let target_sentence = read_line(reader);

    let result = match can_form(&mut sentences, &target_sentence) {
        true => "Possible",
        false => "Impossible",
    };

    write!(writer, "{}", result).expect("Failed to write");
}

fn can_form(sentences: &mut [Vec<String>], target_sentence: &str) -> bool {
    for word in target_sentence.split_whitespace() {
        let mut found = false;
        for sentence in sentences.iter_mut() {
            if sentence
                .first()
                .map_or(false, |first_word| first_word == word)
            {
                sentence.remove(0);
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    sentences.iter().all(|sentence| sentence.is_empty())
}

// https://www.acmicpc.net/problem/14713
// 앵무새
#[test]
fn test_solve14713() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
i want to see you
next week
good luck
i want next good luck week to see you"
                .to_string(),
            want: "Possible".to_string(),
        },
        TestData {
            s: "2
i found
an interesting cave
i found an cave interesting"
                .to_string(),
            want: "Impossible".to_string(),
        },
        TestData {
            s: "2
please
be careful
pen pineapple apple pen"
                .to_string(),
            want: "Impossible".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14713(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to read");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

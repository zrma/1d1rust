use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9226(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut answers = vec![];

    loop {
        let line = read_line(reader);
        if line == "#" {
            break;
        }

        let mut chars = line.chars().collect::<Vec<_>>();
        match chars.iter().position(|&c| is_vowel(c)) {
            None => answers.push(chars.iter().collect::<String>() + "ay"),
            Some(index) => {
                chars.rotate_left(index);
                answers.push(chars.iter().collect::<String>() + "ay");
            }
        }
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

// https://www.acmicpc.net/problem/9226
// noinspection SpellCheckingInspection
// 도깨비말
#[test]
fn test_solve9226() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "frog
apple
pear
#"
            .to_string(),
            want: "ogfray
appleay
earpay"
                .to_string(),
        },
        TestData {
            s: "banana
#"
            .to_string(),
            want: "ananabay".to_string(),
        },
        TestData {
            s: "ph
#"
            .to_string(),
            want: "phay".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve9226(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

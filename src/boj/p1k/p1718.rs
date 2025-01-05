use crate::utils::functions::char_to_index;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1718(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = reader.lines().next().unwrap().unwrap();
    let key = reader.lines().next().unwrap().unwrap();
    let keys_as_bytes = key.as_bytes();

    let mut answers = String::new();
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            answers.push(' ');
            continue;
        }

        let key_char = char::from(keys_as_bytes[i % key.len()]);
        let key_num: u32 = char_to_index(key_char);
        let c_num: u32 = char_to_index(c);

        let ans = if c_num <= key_num {
            26 + c_num - key_num
        } else {
            c_num - key_num
        };
        answers.push(char::from_u32(ans + 96).unwrap());
    }

    writeln!(writer, "{}", answers).unwrap();
}

// https://www.acmicpc.net/problem/1718
// 암호
// noinspection SpellCheckingInspection
#[test]
fn test_solve1718() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "nice day
love"
                .to_string(),
            want: "btgz oet".to_string(),
        },
        TestData {
            s: "          \n ".to_string(),
            want: "          ".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1718(&mut reader, &mut writer);

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

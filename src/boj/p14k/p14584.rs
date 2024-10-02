use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14584(reader: &mut impl BufRead, writer: &mut impl Write) {
    let password = read_line(reader);
    let num_words: usize = read_value(read_line(reader));
    let words = (0..num_words)
        .map(|_| read_line(reader))
        .collect::<Vec<_>>();

    'outer: for shift in 0..26 {
        let decoded: String = password
            .chars()
            .map(|c| (((c as u8 - b'a' + shift) % 26 + b'a') as char))
            .collect::<_>();

        if words.iter().any(|word| decoded.contains(word)) {
            write!(writer, "{}", decoded).expect("Failed to write");
            break 'outer;
        }
    }
}

// https://www.acmicpc.net/problem/14584
// noinspection SpellCheckingInspection
// 암호 해독
#[test]
fn test_solve14584() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "srbvaffefeczevaluxv
3
bake
bread
cookie"
                .to_string(),
            want: "bakejoononlinejudge".to_string(),
        },
        TestData {
            s: "yvccf
2
lo
ll"
            .to_string(),
            want: "hello".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve14584(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

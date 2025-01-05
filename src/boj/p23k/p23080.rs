use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23080(reader: &mut impl BufRead, writer: &mut impl Write) {
    let step = read_value(read_line(reader));
    let encrypted_message = read_line(reader);

    let decrypted_message = decrypt_skitaile(&encrypted_message, step);

    writeln!(writer, "{}", decrypted_message).unwrap();
}

// noinspection SpellCheckingInspection
fn decrypt_skitaile(encrypted_message: &str, step: usize) -> String {
    encrypted_message
        .chars()
        .enumerate()
        .filter(|(index, _)| index % step == 0)
        .map(|(_, character)| character)
        .collect()
}

// https://www.acmicpc.net/problem/23080
// noinspection SpellCheckingInspection
// 스키테일 암호
#[test]
fn test_solve23080() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
iabucdpefcg"
                .to_string(),
            want: "iupc".to_string(),
        },
        TestData {
            s: "4
iabucdpefcg"
                .to_string(),
            want: "icf".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23080(&mut reader, &mut writer);

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

use crate::utils::io::{read_line, read_value};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9322(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value(read_line(reader));

    let mut ans = vec![];
    for _ in 0..num_cases {
        let num_words = read_value(read_line(reader));

        let encryption_key = read_line(reader)
            .split_whitespace()
            .take(num_words)
            .enumerate()
            .map(|(i, s)| (s.to_string(), i))
            .collect::<HashMap<_, _>>();

        let decryption_order = read_line(reader)
            .split_whitespace()
            .take(num_words)
            .map(|s| *encryption_key.get(s).unwrap())
            .collect::<Vec<_>>();

        let encrypted_words = read_line(reader)
            .split_whitespace()
            .take(num_words)
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let mut decrypted_words = vec![""; num_words];
        for (i, &idx) in decryption_order.iter().enumerate() {
            decrypted_words[idx] = &encrypted_words[i];
        }

        ans.push(decrypted_words.join(" "));
    }

    write!(writer, "{}", ans.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/9322
// noinspection SpellCheckingInspection
// 철벽 보안 알고리즘
#[test]
fn test_solve9322() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
4
A B C D
D A B C
C B A P
3
SECURITY THROUGH OBSCURITY
OBSCURITY THROUGH SECURITY
TOMORROW ATTACK WE"
                .to_string(),
            want: "B A P C
WE ATTACK TOMORROW"
                .to_string(),
        },
        TestData {
            s: "1
4
A B C D
D A B C
C B A P"
                .to_string(),
            want: "B A P C".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9322(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

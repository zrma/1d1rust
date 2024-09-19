use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17502(reader: &mut impl BufRead, writer: &mut impl Write) {
    let len: usize = read_value(read_line(reader));
    let s = read_line(reader);
    let s = s.as_bytes();

    let s: String = s
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if c == b'?' {
                if s[len - i - 1] == b'?' {
                    b'a'
                } else {
                    s[len - i - 1]
                }
            } else {
                c
            }
        })
        .map(|c| c as char)
        .collect::<_>();

    write!(writer, "{}", s).expect("Failed to write");
}

// https://www.acmicpc.net/problem/17502
// noinspection SpellCheckingInspection
// 클레어와 팰린드롬
#[test]
fn test_solve17502() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
??ta?or"
                .to_string(),
            want: "rotator".to_string(),
        },
        TestData {
            s: "4
????"
                .to_string(),
            want: "aaaa".to_string(),
        },
        TestData {
            s: "1
?"
            .to_string(),
            want: "a".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve17502(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11094(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let prefix = "Simon says";

    let ans = (0..num_cases)
        .filter_map(|_| {
            let s = read_line(reader);
            if s.starts_with(prefix) {
                Some(s[10..].to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/11094
// 꿍 가라사대
#[test]
fn test_solve11094() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
Simon says smile."
                .to_string(),
            want: " smile.".to_string(),
        },
        TestData {
            s: "3
Simon says raise your right hand.
Lower your right hand.
Simon says raise your left hand."
                .to_string(),
            want: " raise your right hand.
 raise your left hand."
                .to_string(),
        },
        TestData {
            s: "3
Raise your right hand.
Lower your right hand.
Simon says raise your left hand."
                .to_string(),
            want: " raise your left hand.".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11094(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

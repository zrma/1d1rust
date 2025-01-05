use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15881(reader: &mut impl BufRead, writer: &mut impl Write) {
    let len: usize = read_value(read_line(reader));
    let line = read_line(reader);
    let mut count = 0;
    let mut i = 0;
    while i <= len - 4 {
        if &line[i..i + 4] == "pPAp" {
            count += 1;
            i += 4;
        } else {
            i += 1;
        }
    }

    writeln!(writer, "{}", count).unwrap();
}

// https://www.acmicpc.net/problem/15881
// Pen Pineapple Apple Pen
#[test]
fn test_solve15881() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "15
ApPApPpAPpApPAp"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "7
pPApPAp"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "12
pPAppPAppPAp"
                .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15881(&mut reader, &mut writer);

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

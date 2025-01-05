use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15000(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut res = String::new();
    for c in s.chars() {
        if c.is_ascii_lowercase() {
            res.push(c.to_ascii_uppercase());
        }
    }

    writeln!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/15000
// CAPS
// noinspection SpellCheckingInspection
#[test]
fn test_solve15000() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "alert".to_string(),
            want: "ALERT".to_string(),
        },
        TestData {
            s: "earthisunderattack".to_string(),
            want: "EARTHISUNDERATTACK".to_string(),
        },
        TestData {
            s: "canyoupickupsomegroceries".to_string(),
            want: "CANYOUPICKUPSOMEGROCERIES".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15000(&mut reader, &mut writer);

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

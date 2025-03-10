use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27310(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let ans: i32 = s
        .chars()
        .map(|ch| match ch {
            '_' => 6,
            ':' => 2,
            _ => 1,
        })
        .sum();

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/27310
// :chino_shock:
// noinspection SpellCheckingInspection
#[test]
fn test_solve27310() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: ":chino_shock:".to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: ":chinoaww:".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: ":chino_very_shock:".to_string(),
            want: "30".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27310(&mut reader, &mut writer);

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

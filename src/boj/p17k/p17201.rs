use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17201(reader: &mut impl BufRead, writer: &mut impl Write) {
    let _: usize = read_value(read_line(reader));
    let s = read_line(reader);
    let chars: Vec<char> = s.chars().collect();

    let ans = chars.windows(2).all(|w| w[0] != w[1]);

    write!(writer, "{}", if ans { "Yes" } else { "No" }).expect("Failed to write");
}

// https://www.acmicpc.net/problem/17201
// noinspection SpellCheckingInspection
// 자석 체인
#[test]
fn test_solve17201() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
122112"
                .to_string(),
            want: "No".to_string(),
        },
        TestData {
            s: "4
21212121"
                .to_string(),
            want: "Yes".to_string(),
        },
        TestData {
            s: "5
1212121212"
                .to_string(),
            want: "Yes".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve17201(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

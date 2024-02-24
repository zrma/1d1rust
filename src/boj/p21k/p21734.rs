use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve21734(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let ans = s
        .chars()
        .map(repeat_char_based_on_ascii_value)
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).unwrap();
}

fn repeat_char_based_on_ascii_value(c: char) -> String {
    let mut n = c as usize;
    let mut repeat_cnt = 0;
    while n > 0 {
        repeat_cnt += n % 10;
        n /= 10;
    }

    c.to_string().repeat(repeat_cnt)
}

// https://www.acmicpc.net/problem/21734
// noinspection SpellCheckingInspection
// SMUPC의 등장
#[test]
fn test_solve21734() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "smupc".to_string(),
            want: "sssssss
mmmmmmmmmm
uuuuuuuuu
pppp
cccccccccccccccccc"
                .to_string(),
        },
        TestData {
            s: "a".to_string(),
            want: "aaaaaaaaaaaaaaaa".to_string(),
        },
        TestData {
            s: "z".to_string(),
            want: "zzzzz".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve21734(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

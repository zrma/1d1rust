use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17249(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut left = 0;
    let mut right = 0;

    let mut is_left = true;
    for c in s.chars() {
        if c == '(' {
            is_left = false;
        } else if c == '@' {
            if is_left {
                left += 1;
            } else {
                right += 1;
            }
        }
    }

    writeln!(writer, "{} {}", left, right).unwrap();
}

// https://www.acmicpc.net/problem/17249
// 태보태보 총난타
#[test]
fn test_solve17249() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "@===@==@=@==(^0^)==@=@===@".to_string(),
        want: "4 3".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17249(&mut reader, &mut writer);

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

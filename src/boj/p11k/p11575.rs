use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11575(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));

    for _ in 0..t {
        let (a, b) = read_values_as!(read_line(reader), i32, i32);

        let ans: String = read_line(reader)
            .chars()
            .map(|c| {
                let mut c = c as i32 - 'A' as i32;
                c = (a * c + b) % 26;
                (c + 'A' as i32) as u8 as char
            })
            .collect::<_>();

        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/11575
// Affine Cipher
// noinspection SpellCheckingInspection
#[test]
fn test_solve11575() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
3 1
IAMSPY
5 3
ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .to_string(),
            want: "ZBLDUV
DINSXCHMRWBGLQVAFKPUZEJOTY
"
            .to_string(),
        },
        TestData {
            s: "1
3 1
IAMSPY"
                .to_string(),
            want: "ZBLDUV
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11575(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), data.want.trim(), "case {} failed", i);
    }
}

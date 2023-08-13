use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11575(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..t {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        let b = iter.next().unwrap().parse::<i32>().unwrap();

        let s = read_line(reader);
        let ans = s
            .chars()
            .map(|c| {
                let mut c = c as i32 - 'A' as i32;
                c = (a * c + b) % 26;
                (c + 'A' as i32) as u8 as char
            })
            .collect::<String>();

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
    for (i, data) in vec![
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
        assert_eq!(got, data.want, "case {} failed", i);
    }
}

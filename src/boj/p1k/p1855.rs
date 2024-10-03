use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1855(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();
    let s = read_line(reader);
    decrypt_cipher(n, &s, writer);
}

fn decrypt_cipher(n: usize, s: &str, writer: &mut impl Write) {
    let vec = s.as_bytes();
    let div = s.len() / n;

    for i in 0..n {
        for j in 0..div {
            let idx = if j % 2 == 0 {
                j * n + i
            } else {
                (j + 1) * n - i - 1
            };
            write!(writer, "{}", char::from(vec[idx])).expect("write! should work");
        }
    }
}

// https://www.acmicpc.net/problem/1855
// 암호
// noinspection SpellCheckingInspection
#[test]
fn test_solve1855() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
aeijfbcgklhd"
                .to_string(),
            want: "abcdefghijkl".to_string(),
        },
        TestData {
            s: "4
adgjkhebcfil"
                .to_string(),
            want: "abcdefghijkl".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1855(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9324(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..n {
        let s = read_line(reader);
        let mut cnt = [0; 26];
        let mut is_fake = false;

        let chars = s.as_bytes();
        let len = chars.len();

        let mut idx = 0;
        while idx < len {
            let ch = chars[idx];
            let ch_idx = (ch - b'A') as usize;
            cnt[ch_idx] += 1;

            if cnt[ch_idx] % 3 == 0 {
                if idx + 1 >= len || chars[idx + 1] != ch {
                    is_fake = true;
                    break;
                }
                idx += 1;
            }

            idx += 1;
        }

        writeln!(writer, "{}", if is_fake { "FAKE" } else { "OK" }).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/9324
// 진짜 메시지
// noinspection SpellCheckingInspection
#[test]
fn test_solve9324() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
BAPC
AABA
ABCABCBBAAACC"
                .to_string(),
            want: "OK
FAKE
OK
"
            .to_string(),
        },
        TestData {
            s: "1
AAAABBBBCCCCDDDDEEEEFFFF"
                .to_string(),
            want: "OK
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9324(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

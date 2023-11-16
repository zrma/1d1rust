use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11091(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    for _ in 0..n {
        let s = read_line(reader);
        let mut arr = [false; 26];

        s.chars()
            .filter(|&ch| ch.is_ascii_alphabetic())
            .map(|ch| ch.to_ascii_lowercase())
            .for_each(|ch| {
                arr[(ch as u8 - b'a') as usize] = true;
            });

        let ans = match arr.iter().all(|&b| b) {
            true => "pangram".to_string(),
            false => {
                "missing ".to_string()
                    + &arr
                        .iter()
                        .enumerate()
                        .filter(|(_, &b)| !b)
                        .map(|(i, _)| (b'a' + i as u8) as char)
                        .collect::<String>()
            }
        };

        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/11091
// 알파벳 전부 쓰기
#[test]
fn test_solve11091() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
The quick brown fox jumps over the lazy dog.
ZYXW, vu TSR Ponm lkj ihgfd CBA.
.,?!'\" 92384 abcde FGHIJ"
                .to_string(),
            want: "pangram
missing eq
missing klmnopqrstuvwxyz
"
            .to_string(),
        },
        TestData {
            s: "1
The quick brown fox jumps over the lazy dog."
                .to_string(),
            want: "pangram
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11091(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

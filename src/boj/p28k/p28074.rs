use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28074(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut arr = vec![true; 26];
    [b'M', b'O', b'B', b'I', b'S']
        .iter()
        .for_each(|&x| arr[(x - b'A') as usize] = false);

    let ans = s
        .chars()
        .fold(arr, |mut acc, x| {
            acc[(x as u8 - b'A') as usize] = true;
            acc
        })
        .iter()
        .all(|&x| x);

    if ans {
        write!(writer, "YES").unwrap();
    } else {
        write!(writer, "NO").unwrap();
    }
}

// https://www.acmicpc.net/problem/28074
// 모비스
// noinspection SpellCheckingInspection
#[test]
fn test_solve28074() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "MOIISB".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "MOBI".to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "INHACTP".to_string(),
            want: "NO".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve28074(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

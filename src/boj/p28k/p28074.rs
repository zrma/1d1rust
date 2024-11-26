use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28074(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut arr = vec![true; 26];
    [b'M', b'O', b'B', b'I', b'S'].iter().for_each(|&x| {
        if let Some(idx) = byte_to_index(x) {
            arr[idx] = false;
        }
    });

    let ans = s
        .chars()
        .fold(arr, |mut acc, x| {
            if let Some(idx) = char_to_index(x) {
                acc[idx] = true;
            }
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

fn byte_to_index(x: u8) -> Option<usize> {
    if x.is_ascii_uppercase() {
        Option::from(usize::from(x - b'A'))
    } else {
        None
    }
}

fn char_to_index(ch: char) -> Option<usize> {
    if ch.is_ascii_alphabetic() {
        let upper = ch.to_ascii_uppercase();
        let x = upper as u8;
        byte_to_index(x)
    } else {
        None
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
    for (i, data) in [
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

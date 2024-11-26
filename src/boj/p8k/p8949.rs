use crate::read_values_as;
use crate::utils::functions::char_to_index;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8949(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = read_values_as!(read_line(reader), String, String);

    let mut ans = vec![];
    let mut a_iter = a.chars().rev().peekable();
    let mut b_iter = b.chars().rev().peekable();

    loop {
        let a_char = a_iter.next().unwrap_or('0');
        let b_char = b_iter.next().unwrap_or('0');

        ans.push(char_to_index::<u32>(a_char) + char_to_index::<u32>(b_char));

        if a_iter.peek().is_none() && b_iter.peek().is_none() {
            break;
        }
    }

    for i in ans.iter().rev() {
        write!(writer, "{}", i).unwrap();
    }
}

// https://www.acmicpc.net/problem/8949
// 대충 더해
#[test]
fn test_solve8949() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "512 444".to_string(),
            want: "956".to_string(),
        },
        TestData {
            s: "123 2495".to_string(),
            want: "25118".to_string(),
        },
        TestData {
            s: "99999 99999".to_string(),
            want: "1818181818".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8949(&mut reader, &mut writer);

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

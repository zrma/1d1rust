use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8949(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let a = iter.next().unwrap().to_string();
        let b = iter.next().unwrap().to_string();
        (a, b)
    };

    let mut ans = vec![];
    let mut a_iter = a.chars().rev().peekable();
    let mut b_iter = b.chars().rev().peekable();

    loop {
        let a_char = a_iter.next().unwrap_or('0');
        let b_char = b_iter.next().unwrap_or('0');

        ans.push(a_char.to_digit(10).unwrap() + b_char.to_digit(10).unwrap());

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
    for (i, data) in vec![
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

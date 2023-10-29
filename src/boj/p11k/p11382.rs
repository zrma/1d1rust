use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11382(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c) = read_values!(read_line(reader), String, String, String);

    let mut res = String::new();
    let mut carry = 0;
    let mut a_iter = a.chars().rev();
    let mut b_iter = b.chars().rev();
    let mut c_iter = c.chars().rev();

    loop {
        let a_char = a_iter.next();
        let b_char = b_iter.next();
        let c_char = c_iter.next();

        if a_char.is_none() && b_char.is_none() && c_char.is_none() {
            break;
        }

        let a = a_char.unwrap_or('0').to_digit(10).unwrap();
        let b = b_char.unwrap_or('0').to_digit(10).unwrap();
        let c = c_char.unwrap_or('0').to_digit(10).unwrap();

        let sum = a + b + c + carry;
        carry = sum / 10;
        res.push_str(&format!("{}", sum % 10));
    }

    if carry > 0 {
        res.push_str(&format!("{}", carry));
    }

    res = res.chars().rev().collect();
    write!(writer, "{}", res).unwrap();
}

#[allow(dead_code)]
fn solve11382simple(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let numbers = s
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let sum = numbers.iter().sum::<u64>();
    write!(writer, "{}", sum).unwrap();
}

// https://www.acmicpc.net/problem/11382
// 꼬마 정민
#[test]
fn test_solve11382() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "77 77 7777".to_string(),
            want: "7931".to_string(),
        },
        TestData {
            s: "7777 77 77".to_string(),
            want: "7931".to_string(),
        },
        TestData {
            s: "1 1 1".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: " 1000000000000 1000000000000 1000000000000".to_string(),
            want: "3000000000000".to_string(),
        },
        TestData {
            s: " 10000000000 100000000000 1000000000000".to_string(),
            want: "1110000000000".to_string(),
        },
        TestData {
            s: " 9999999999999 9999999999999 9999999999999".to_string(),
            want: "29999999999997".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve11382(&mut reader, &mut writer);

            let got = String::from_utf8(writer).unwrap();
            assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
        }

        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve11382simple(&mut reader, &mut writer);

            let got = String::from_utf8(writer).unwrap();
            assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
        }
    }
}

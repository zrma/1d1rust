use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15740(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b): (String, String) = read_values_as!(read_line(reader), String, String);
    let result = add_big_numbers(&a, &b);
    writeln!(writer, "{}", result).unwrap();
}

fn add_big_numbers(a: &str, b: &str) -> String {
    // 음수 처리
    let a_negative = a.starts_with('-');
    let b_negative = b.starts_with('-');
    let a_num = if a_negative { &a[1..] } else { a };
    let b_num = if b_negative { &b[1..] } else { b };

    let a_digits: Vec<i32> = a_num.bytes().rev().map(|c| (c - b'0') as i32).collect();
    let b_digits: Vec<i32> = b_num.bytes().rev().map(|c| (c - b'0') as i32).collect();

    let mut result = Vec::new();
    let mut carry = 0;
    let mut i = 0;

    while i < a_digits.len() || i < b_digits.len() || carry != 0 {
        let a_val = if i < a_digits.len() { a_digits[i] } else { 0 };
        let b_val = if i < b_digits.len() { b_digits[i] } else { 0 };
        let sum = a_val + b_val + carry;
        result.push(sum % 10);
        carry = sum / 10;
        i += 1;
    }

    if result.is_empty() {
        return "0".to_string();
    }

    let mut ans = if a_negative && b_negative {
        "-".to_string()
    } else {
        String::new()
    };
    for digit in result.iter().rev() {
        ans.push((b'0' + *digit as u8) as char);
    }
    ans
}

// https://www.acmicpc.net/problem/15740
// A+B - 9
#[test]
fn test_solve15740() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "1 2".to_string(),
            want: "3".to_string(),
        },
        TestCase {
            s: "-1 -2".to_string(),
            want: "-3".to_string(),
        },
        TestCase {
            s: "123456789123456789123456789 987654321987654321987654321".to_string(),
            want: "1111111111111111111111111110".to_string(),
        },
        TestCase {
            s: "-123456789123456789123456789 -987654321987654321987654321".to_string(),
            want: "-1111111111111111111111111110".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15740(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

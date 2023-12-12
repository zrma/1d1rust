use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15353(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let (a, b) = read_values!(line, String, String);

    let mut res = String::new();
    let mut carry = 0;
    let mut a_iter = a.chars().rev();
    let mut b_iter = b.chars().rev();

    loop {
        let a_char = a_iter.next();
        let b_char = b_iter.next();

        if a_char.is_none() && b_char.is_none() {
            break;
        }

        let a = a_char.unwrap_or('0').to_digit(10).unwrap();
        let b = b_char.unwrap_or('0').to_digit(10).unwrap();

        let sum = a + b + carry;
        carry = sum / 10;
        res.push_str(&format!("{}", sum % 10));
    }

    if carry > 0 {
        res.push_str(&format!("{}", carry));
    }

    res = res.chars().rev().collect();
    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/15353
// 큰 수 A+B (2)
#[test]
fn test_solve15353() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9223372036854775807 9223372036854775808".to_string(),
            want: "18446744073709551615".to_string(),
        },
        TestData {
            s: "19223372036854775807 9223372036854775808".to_string(),
            want: "28446744073709551615".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15353(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

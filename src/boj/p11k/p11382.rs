use crate::read_values_as;
use crate::utils::functions::char_to_index;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11382(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c) = read_values_as!(read_line(reader), String, String, String);

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

        let a: u32 = char_to_index(a_char.unwrap_or('0'));
        let b: u32 = char_to_index(b_char.unwrap_or('0'));
        let c: u32 = char_to_index(c_char.unwrap_or('0'));

        let sum = a + b + c + carry;
        carry = sum / 10;
        res.push_str(&format!("{}", sum % 10));
    }

    if carry > 0 {
        res.push_str(&format!("{}", carry));
    }

    res = res.chars().rev().collect();
    write!(writer, "{}", res).expect("Failed to write");
}

#[allow(dead_code)]
fn solve11382simple(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let numbers: Vec<u64> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let sum: u64 = numbers.iter().sum();
    write!(writer, "{}", sum).expect("Failed to write");
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

            let got = String::from_utf8(writer).expect("writer should be a valid string");
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }

        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve11382simple(&mut reader, &mut writer);

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
}

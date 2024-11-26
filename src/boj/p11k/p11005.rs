use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11005(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    let res = convert_to_base(nums[0], nums[1]);

    write!(writer, "{}", res).expect("Failed to write");
}

fn convert_to_base(mut n: i32, b: i32) -> String {
    let mut res = String::new();
    while n > 0 {
        let r = n % b;
        let c = if r < 10 {
            (r as u8 + b'0') as char
        } else {
            (r as u8 - 10 + b'A') as char
        };
        res.push(c);
        n /= b;
    }
    res.chars().rev().collect()
}

// https://www.acmicpc.net/problem/11005
// 진법 변환 2
#[test]
fn test_solve11005() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "60466175 36".to_string(),
            want: "ZZZZZ".to_string(),
        },
        TestData {
            s: "123 2".to_string(),
            want: "1111011".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11005(&mut reader, &mut writer);

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

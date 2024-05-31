use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4388(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut results = vec![];
    loop {
        let (mut a, mut b) = read_values_as!(read_line(reader), i32, i32);
        if a == 0 && b == 0 {
            break;
        }

        let mut carry = 0;
        let mut carry_count = 0;
        while a > 0 || b > 0 {
            let sum = a % 10 + b % 10 + carry;
            if sum >= 10 {
                carry_count += 1;
                carry = 1;
            } else {
                carry = 0;
            }
            a /= 10;
            b /= 10;
        }

        results.push(carry_count.to_string());
    }

    write!(writer, "{}", results.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/4388
// 받아올림
#[test]
fn test_solve4388() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "123 456
555 555
123 594
0 0"
            .to_string(),
            want: "0
3
1"
            .to_string(),
        },
        TestData {
            s: "999 999
1 1
0 0"
            .to_string(),
            want: "3
0"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4388(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

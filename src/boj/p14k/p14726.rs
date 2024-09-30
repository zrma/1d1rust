use crate::utils::functions::char_to_index;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14726(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    for _ in 0..t {
        let s = read_line(reader);
        let res = s.chars().enumerate().fold(0, |acc, (i, c)| {
            let mut x: u32 = char_to_index(c);
            if i % 2 == 0 {
                x *= 2;
                x = x / 10 + x % 10;
            }
            acc + x
        });

        if res % 10 == 0 {
            writeln!(writer, "T").expect("Failed to write");
        } else {
            writeln!(writer, "F").expect("Failed to write");
        }
    }
}

// https://www.acmicpc.net/problem/14726
// 신용카드 판별
#[test]
fn test_solve14726() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
2720992711828767
3444063910462763
6011733895106094"
                .to_string(),
            want: "T
F
T
"
            .to_string(),
        },
        TestData {
            s: "1
2720992711828767"
                .to_string(),
            want: "T
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14726(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2998(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let padding = (3 - (line.len() % 3)) % 3;
    let padded_line = format!("{:0>width$}", line, width = line.len() + padding);

    let res = padded_line
        .as_bytes()
        .chunks(3)
        .fold(String::new(), |mut acc, chunk| {
            let n = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 2).unwrap();
            acc.push_str(&format!("{}", n));
            acc
        });

    writeln!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/2998
// 8진수
#[test]
fn test_solve2998() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1010".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "11001100".to_string(),
            want: "314".to_string(),
        },
        TestData {
            s: "010   ".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "0".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2998(&mut reader, &mut writer);

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

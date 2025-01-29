use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5341(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let n: i32 = read_value(read_line(reader));
        if n == 0 {
            break;
        }

        // n*(n+1)/2 공식 사용
        let result = n * (n + 1) / 2;
        writeln!(writer, "{}", result).unwrap();
    }
}

// https://www.acmicpc.net/problem/5341
// Pyramids
#[test]
fn test_solve5341() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestCase {
        s: "1
2
3
4
10
0"
        .to_string(),
        want: "1
3
6
10
55"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5341(&mut reader, &mut writer);

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

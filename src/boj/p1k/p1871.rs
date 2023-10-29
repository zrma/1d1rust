use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1871(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).trim().parse::<i64>().unwrap();
    for _ in 0..n {
        let line = read_line(reader);
        let mut iter = line.split('-');
        let (left, right) = (iter.next().unwrap(), iter.next().unwrap().trim());

        let left_value = left
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| ((c as u8 - b'A') as usize) * 26usize.pow(i as u32))
            .sum::<usize>();

        let right_value = right.parse::<i64>().unwrap();

        let diff = (left_value as i64 - right_value).abs();
        let result = if diff <= 100 { "nice" } else { "not nice" };

        writeln!(writer, "{}", result).unwrap();
    }
}

// https://www.acmicpc.net/problem/1871
// 좋은 자동차 번호판
#[test]
fn test_solve1871() {
    struct TestData {
        s: String,
        want: String,
    }
    for data in [
        TestData {
            s: "2
ABC-0123
AAA-9999"
                .to_string(),
            want: "nice
not nice
"
            .to_string(),
        },
        TestData {
            s: "1
ABC-0123"
                .to_string(),
            want: "nice
"
            .to_string(),
        },
        TestData {
            s: "1
AAA-9999"
                .to_string(),
            want: "not nice
"
            .to_string(),
        },
    ] {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1871(&mut reader, &mut writer);
        assert_eq!(String::from_utf8(writer).unwrap(), data.want);
    }
}

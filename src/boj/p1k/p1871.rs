use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1871(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut answers = Vec::with_capacity(n);
    for _ in 0..n {
        let line = read_line(reader);
        let mut iter = line.split('-');
        let (left, right) = (iter.next().unwrap(), iter.next().unwrap().trim());

        let left_value: usize = left
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| ((c as u8 - b'A') as usize) * 26usize.pow(i as u32))
            .sum();

        let right_value: i64 = right.parse().unwrap();

        let diff = (left_value as i64 - right_value).abs();
        let ans = if diff <= 100 { "nice" } else { "not nice" };
        answers.push(ans);
    }

    writeln!(writer, "{}", answers.join("\n")).expect("write! should work");
}

// https://www.acmicpc.net/problem/1871
// 좋은 자동차 번호판
#[test]
fn test_solve1871() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
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
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1871(&mut reader, &mut writer);

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

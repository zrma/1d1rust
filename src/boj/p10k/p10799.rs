use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10799(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);
    let mut stack = Vec::new();
    let mut count = 0;

    for (i, c) in line.chars().enumerate() {
        match c {
            '(' => stack.push(i),
            ')' => {
                if let Some(last_open) = stack.pop() {
                    count += if last_open + 1 == i { stack.len() } else { 1 };
                }
            }
            _ => {}
        }
    }

    writeln!(writer, "{}", count).unwrap();
}

// https://www.acmicpc.net/problem/10799
// 쇠막대기
#[test]
fn test_solve10799() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "()(((()())(())()))(())".to_string(),
            want: "17".to_string(),
        },
        TestData {
            s: "(((()(()()))(())()))(()())".to_string(),
            want: "24".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10799(&mut reader, &mut writer);

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

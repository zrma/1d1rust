use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4889(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut case = 1;
    loop {
        let mut stack = Vec::new();
        let mut res = 0;

        let s = read_line(reader);
        if s.contains('-') {
            break;
        }

        for c in s.chars() {
            if c == '{' {
                stack.push(c);
            } else if stack.is_empty() {
                stack.push('{');
                res += 1;
            } else {
                stack.pop();
            }
        }

        res += stack.len() / 2;

        writeln!(writer, "{}. {}", case, res).unwrap();
        case += 1;
    }
}

// https://www.acmicpc.net/problem/4889
// 안정적인 문자열
#[test]
fn test_solve4889() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "}{
{}{}{}
{{{}
---"
        .to_string(),
        want: "1. 2
2. 0
3. 1
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4889(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

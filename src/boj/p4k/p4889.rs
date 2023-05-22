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
fn test_4889() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
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
        use std::io::Cursor;
        let mut reader = Cursor::new(&data.s);
        let mut writer = Cursor::new(Vec::new());
        solve4889(&mut reader, &mut writer);

        let got = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(data.want, got, "case {}", i);
    }
}

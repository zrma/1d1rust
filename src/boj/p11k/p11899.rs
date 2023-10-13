use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11899(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut stack = vec![];

    for c in s.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            if !stack.is_empty() && stack[stack.len() - 1] == '(' {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
    }

    write!(writer, "{}", stack.len()).unwrap();
}

// https://www.acmicpc.net/problem/11899
// 괄호 끼워넣기
#[test]
fn test_solve11899() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: ")))()".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: ")(()".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "))()((".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: ")(()(()))".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "(((((((((()".to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "())))))))))".to_string(),
            want: "9".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11899(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

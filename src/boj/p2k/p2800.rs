use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2800(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut stack = vec![];
    let mut pairs = vec![];

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            let j = stack.pop().unwrap();
            pairs.push((j, i));
        }
    }

    let mut ans = vec![];
    for i in 1..(1 << pairs.len()) {
        let mut contains = vec![true; s.len()];
        for j in 0..pairs.len() {
            if i & (1 << j) != 0 {
                contains[pairs[j].0] = false;
                contains[pairs[j].1] = false;
            }
        }
        let t: String = s
            .chars()
            .enumerate()
            .filter(|&(i, _)| contains[i])
            .map(|(_, c)| c)
            .collect();
        ans.push(t);
    }

    ans.sort();
    ans.dedup();

    for a in ans {
        writeln!(writer, "{}", a).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/2800
// 괄호 제거
#[test]
fn test_solve2800() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "(0/(0))".to_string(),
            want: "(0/0)
0/(0)
0/0
"
            .to_string(),
        },
        TestData {
            s: "(2+(2*2)+2)".to_string(),
            want: "(2+2*2+2)
2+(2*2)+2
2+2*2+2
"
            .to_string(),
        },
        TestData {
            s: "(1+(2*(3+4)))".to_string(),
            want: "(1+(2*3+4))
(1+2*(3+4))
(1+2*3+4)
1+(2*(3+4))
1+(2*3+4)
1+2*(3+4)
1+2*3+4
"
            .to_string(),
        },
        TestData {
            s: "(((1)))(2)".to_string(),
            want: "(((1)))2
((1))(2)
((1))2
(1)(2)
(1)2
1(2)
12
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2800(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

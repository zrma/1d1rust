use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20528(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    let mut words = read_line(reader)
        .split_whitespace()
        .map(|s| s.to_string())
        .take(n)
        .collect::<Vec<_>>();

    words.sort();

    let mut ans = 1;
    for i in 1..n {
        if words[i - 1].chars().last().unwrap() != words[i].chars().next().unwrap() {
            ans = 0;
            break;
        }
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/20528
// 끝말잇기
#[test]
fn test_solve20528() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
pqqp pqpqp pbbbp"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3
aba c dd"
                .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20528(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

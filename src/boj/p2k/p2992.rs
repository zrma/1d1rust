use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2992(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let numbers = s
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut results = vec![];
    let mut used = vec![false; numbers.len()];
    let mut buf = vec![];
    solve2992_rec(&numbers, &mut used, &mut buf, &mut results);

    results.sort();
    let mut ans = 0;
    for a in results {
        if a > numbers.iter().fold(0, |acc, &n| acc * 10 + n) {
            ans = a;
            break;
        }
    }

    write!(writer, "{}", ans).unwrap();
}

fn solve2992_rec(numbers: &[u32], used: &mut [bool], buf: &mut Vec<u32>, answers: &mut Vec<u32>) {
    if buf.len() == numbers.len() {
        answers.push(buf.iter().fold(0, |acc, &n| acc * 10 + n));
        return;
    }

    for i in 0..numbers.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        buf.push(numbers[i]);
        solve2992_rec(numbers, used, buf, answers);
        buf.pop();
        used[i] = false;
    }
}

// https://www.acmicpc.net/problem/2992
// 크면서 작은 수
#[test]
fn test_solve2992() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "156".to_string(),
            want: "165".to_string(),
        },
        TestData {
            s: "330".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1000".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "999".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1001".to_string(),
            want: "1010".to_string(),
        },
        TestData {
            s: "27711".to_string(),
            want: "71127".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2992(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

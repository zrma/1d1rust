use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11383(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, _) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let m = iter.next().unwrap().parse::<usize>().unwrap();
        (n, m)
    };

    let src = {
        let mut src = vec![];
        for _ in 0..n {
            let s = read_line(reader);
            src.push(s);
        }
        src
    };

    //noinspection SpellCheckingInspection
    const OK: &str = "Eyfa";
    //noinspection SpellCheckingInspection
    const NG: &str = "Not Eyfa";

    for s in src.iter() {
        let arr = s.as_bytes();

        let cur = read_line(reader);
        let mut iter = cur.chars();

        for (j, c) in iter.by_ref().enumerate() {
            if c != arr[j / 2] as char {
                write!(writer, "{}", NG).unwrap();
                return;
            }
        }
    }
    write!(writer, "{}", OK).unwrap();
}

// https://www.acmicpc.net/problem/11383
// 뚊
// noinspection SpellCheckingInspection
#[test]
fn test_solve11383() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "1 5
ABCDE
AABBCCDDEE"
                .to_string(),
            want: "Eyfa".to_string(),
        },
        TestData {
            s: "1 5
ABCDE
AABBCCDDEF"
                .to_string(),
            want: "Not Eyfa".to_string(),
        },
        TestData {
            s: "2 2
AB
CD
AABB
CCDD"
                .to_string(),
            want: "Eyfa".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11383(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

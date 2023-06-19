use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2501(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut iter = s.split_whitespace();
    let n = iter.next().unwrap().parse::<i64>().unwrap();
    let k = iter.next().unwrap().parse::<i64>().unwrap();

    let mut cnt = 0;
    for i in 1..=n {
        if n % i == 0 {
            cnt += 1;
            if cnt == k {
                write!(writer, "{}", i).unwrap();
                return;
            }
        }
    }
    write!(writer, "0").unwrap();
}

// https://www.acmicpc.net/problem/2501
// 약수 구하기
#[test]
fn test_solve2501() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "6 3".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "25 4".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "2735 1".to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2501(&mut reader, &mut writer);
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, data.want, "failed at {}th", i);
    }
}

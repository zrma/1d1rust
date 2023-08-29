use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18312(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let k = iter.next().unwrap().parse::<usize>().unwrap();
        (n, k)
    };

    let mut ans = 0;
    for h in 0..=n {
        for m in 0..60 {
            for s in 0..60 {
                if format!("{:02}{:02}{:02}", h, m, s).contains(&format!("{}", k)) {
                    ans += 1;
                }
            }
        }
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/18312
// 시각
#[test]
fn test_solve18312() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "5 3".to_string(),
            want: "11475".to_string(),
        },
        TestData {
            s: "0 0".to_string(),
            want: "3600".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18312(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
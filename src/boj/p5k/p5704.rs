use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5704(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let s = read_line(reader);
        if s == "*" {
            break;
        }

        let mut check = vec![false; 26];
        for c in s.chars() {
            if c != ' ' {
                check[c as usize - 'a' as usize] = true;
            }
        }

        let ans = check.iter().all(|&x| x);
        writeln!(writer, "{}", if ans { "Y" } else { "N" }).unwrap();
    }
}

// https://www.acmicpc.net/problem/5704
// 팬그램
#[test]
fn test_solve5704() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
        s: "jackdawf loves my big quartz sphinx
abcdefghijklmnopqrstuvwxyz
hello world
*"
        .to_string(),
        want: "Y
Y
N
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5704(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

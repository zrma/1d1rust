use crate::utils::functions::char_to_index;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9047(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    let mut ans = Vec::with_capacity(t);
    for _ in 0..t {
        let mut s: String = read_value(read_line(reader));
        let mut cnt = 0;
        while s != "6174" {
            let mut digits: Vec<u32> = s.chars().map(char_to_index).collect();
            digits.sort_unstable();
            let asc = digits.iter().fold(0, |acc, &x| acc * 10 + x);
            let desc = digits.iter().rev().fold(0, |acc, &x| acc * 10 + x);
            s = format!("{:0>4}", desc - asc);
            cnt += 1;
        }
        ans.push(cnt.to_string());
    }

    write!(writer, "{}", ans.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/9047
// 6174
#[test]
fn test_solve9047() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
6174
1789
2005"
                .to_string(),
            want: "0
3
7"
            .to_string(),
        },
        TestData {
            s: "6
6174
4176
6264
3996
7443
8172"
                .to_string(),
            want: "0
1
2
3
4
5"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9047(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

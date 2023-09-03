use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10174(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..n {
        let s = read_line(reader);
        let arr = s.as_bytes();

        let mut ans = true;
        for i in 0..s.len() / 2 {
            if arr[i].to_ascii_lowercase() != arr[s.len() - 1 - i].to_ascii_lowercase() {
                ans = false;
                break;
            }
        }

        writeln!(writer, "{}", if ans { "Yes" } else { "No" }).unwrap();
    }
}

// https://www.acmicpc.net/problem/10174
// 팰린드롬
#[test]
fn test_solve10174() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "6
Nat tan
Palindrome
123454321
Dogs and Cats
**()()**
1 221"
                .to_string(),
            want: "Yes
No
Yes
No
No
No
"
            .to_string(),
        },
        TestData {
            s: "1
Nat tan"
                .to_string(),
            want: "Yes
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10174(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

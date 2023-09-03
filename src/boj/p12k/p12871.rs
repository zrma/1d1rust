use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12871(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (s1, s2) = {
        let s1 = read_line(reader);
        let s2 = read_line(reader);

        if s1.len() > s2.len() {
            (s2, s1)
        } else {
            (s1, s2)
        }
    };

    let len1 = s1.len();
    let len2 = s2.len();
    let lcm = lcm(len1, len2);

    let mut ans = 1;

    let arr1 = s1.as_bytes();
    let arr2 = s2.as_bytes();
    for i in 0..lcm {
        if arr1[i % len1] != arr2[i % len2] {
            ans = 0;
            break;
        }
    }

    write!(writer, "{}", ans).unwrap();
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

// https://www.acmicpc.net/problem/12871
// 무한 문자열
// noinspection SpellCheckingInspection
#[test]
fn test_solve12871() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "ab
abab"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "abc
bca"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "abcabc
abc"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "abcab
abc"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "aa
aaaaa"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve12871(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

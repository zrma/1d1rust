use crate::utils::functions::check_palindrome_nth;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8892(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_line(reader).parse().unwrap();

    for _ in 0..t {
        let n: usize = read_line(reader).parse().unwrap();
        let words: Vec<Vec<u8>> = (0..n)
            .map(|_| read_line(reader).as_bytes().to_vec())
            .collect();

        let mut ans = String::new();
        for i in 0..n {
            for j in i + 1..n {
                let mut word = vec![];
                word.extend_from_slice(&words[i]);
                word.extend_from_slice(&words[j]);
                if is_palindrome(&word) {
                    ans = String::from_utf8(word).unwrap();
                    break;
                }
                word.clear();
                word.extend_from_slice(&words[j]);
                word.extend_from_slice(&words[i]);
                if is_palindrome(&word) {
                    ans = String::from_utf8(word).unwrap();
                    break;
                }
            }
            if !ans.is_empty() {
                break;
            }
        }

        if ans.is_empty() {
            writeln!(writer, "0").expect("Failed to write");
        } else {
            writeln!(writer, "{}", ans).expect("Failed to write");
        }
    }
}

fn is_palindrome(s: &[u8]) -> bool {
    for i in 0..s.len() / 2 {
        if !check_palindrome_nth(s, i) {
            return false;
        }
    }
    true
}

// https://www.acmicpc.net/problem/8892
// 팰린드롬
// noinspection SpellCheckingInspection
#[test]
fn test_solve8892() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
5
aaba
ba
ababa
bbaa
baaba
3
abc
bcd
cde"
            .to_string(),
            want: "abababa
0
"
            .to_string(),
        },
        TestData {
            s: "1
5
aaba
ba
ababa
bbaa
baaba"
                .to_string(),
            want: "abababa
"
            .to_string(),
        },
        TestData {
            s: "1
3
abc
bcd
cde"
            .to_string(),
            want: "0
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8892(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

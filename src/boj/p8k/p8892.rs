use crate::utils::functions::check_palindrome_nth;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8892(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();

    let t = s.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        s.clear();
        reader.read_line(&mut s).unwrap();
        let n = s.trim().parse::<usize>().unwrap();

        let mut words = vec![];
        for _ in 0..n {
            s.clear();
            reader.read_line(&mut s).unwrap();
            words.push(s.trim().to_string());
        }

        let mut ans = String::new();
        for i in 0..n {
            for j in i + 1..n {
                let mut word = words[i].chars().collect::<Vec<_>>();
                word.append(&mut words[j].chars().collect::<Vec<_>>());
                if is_palindrome(&word) {
                    ans = word.iter().collect::<String>();
                    break;
                }

                word = words[j].chars().collect::<Vec<_>>();
                word.append(&mut words[i].chars().collect::<Vec<_>>());
                if is_palindrome(&word) {
                    ans = word.iter().collect::<String>();
                    break;
                }
            }
            if !ans.is_empty() {
                break;
            }
        }

        if ans.is_empty() {
            writeln!(writer, "0").unwrap();
        } else {
            writeln!(writer, "{}", ans).unwrap();
        }
    }
}

fn is_palindrome(s: &Vec<char>) -> bool {
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
    for (i, data) in vec![
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

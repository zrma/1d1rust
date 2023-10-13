use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1141(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let mut strings = vec![];
    strings.reserve(n);
    for _ in 0..n {
        strings.push(read_line(reader));
    }

    strings.sort();

    let ans = count_unique_prefixes(&strings);

    write!(writer, "{}", ans).unwrap();
}

fn count_unique_prefixes(words: &[String]) -> usize {
    let mut unique_prefixes = 0;

    let mut prev_prefix: Option<&String> = None;

    for word in words {
        if let Some(prefix) = prev_prefix {
            if !word.starts_with(prefix) {
                unique_prefixes += 1;
            }
        } else {
            unique_prefixes += 1;
        }

        prev_prefix = Some(word);
    }

    unique_prefixes
}

// https://www.acmicpc.net/problem/1141
// 접두사
#[test]
fn test_solve1141() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6
hello
hi
h
run
rerun
running"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "6
a
b
cba
cbc
cbb
ccc"
            .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "6
a
ab
abc
abcd
abcde
abcdef"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3
topcoder
topcoder
topcoding"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1141(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

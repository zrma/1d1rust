use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9494(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut ans = Vec::new();
    loop {
        let n = read_value(read_line(reader));
        if n == 0 {
            break;
        }

        let mut cursor = 0;
        for i in 0..n {
            let mut s = String::new();
            reader.read_line(&mut s).unwrap();

            if i == 0 {
                cursor += count_leading_spaces(&s);
            }

            cursor = update_cursor(&s, cursor);
        }
        ans.push((cursor + 1).to_string());
    }
    writeln!(writer, "{}", ans.join("\n")).unwrap();
}

fn count_leading_spaces(s: &str) -> usize {
    s.chars().take_while(|&c| c == ' ').count()
}

fn update_cursor(s: &str, cursor: usize) -> usize {
    s.trim_end()
        .chars()
        .skip(cursor)
        .take_while(|&c| c != ' ')
        .count()
        + cursor
}

// https://www.acmicpc.net/problem/9494
// noinspection SpellCheckingInspection
// Text Roll
#[test]
fn test_solve9494() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
Lorem ipsum dolor sit amet, consectetur adipisicing elit,
sed do eiusmod tempor incididunt ut labore et dolore magna
aliqua. Ut enim ad minim veniam, quis nostrud exercitation
ullamco laboris nisi ut aliquip ex ea commodo consequat.
3
Supercalifragilisticexpialidocious! Can you handle
short
lines?
0"
            .to_string(),
            want: "8
36"
            .to_string(),
        },
        TestData {
            s: "4
Loremipsumdolorsitamet,consecteturadipisicingelit,
sed do eiusmod tempor incididunt ut labore et dolore magna
aliqua. Ut enim ad minim veniam, quis nostrud exercitation
ullamco laboris nisi ut aliquip ex ea commodo consequat.
4
A AA
A AA
A AA
A AA
1
 abcde
5
    a
   b
  c
 d
e
3
 The quick brown fox
The quick brown fox
 The quick brown fox
0"
            .to_string(),
            want: "59
2
7
6
11"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9494(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

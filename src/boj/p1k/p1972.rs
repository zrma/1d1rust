use crate::utils::io::read_line;
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1972(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut ans = vec![];
    loop {
        let s = read_line(reader);
        if s == "*" {
            break;
        }
        let is_surprising = is_string_surprising(&s);
        ans.push(format!(
            "{} is {}surprising.",
            s,
            if is_surprising { "" } else { "NOT " }
        ));
    }

    writeln!(writer, "{}", ans.join("\n")).unwrap();
}

fn is_string_surprising(s: &str) -> bool {
    for k in 1..s.len() {
        let mut seen = HashSet::new();
        for (i, window) in s.chars().zip(s.chars().skip(k)).enumerate() {
            let key = format!("{}{}", window.0, window.1);
            if !seen.insert(key) {
                return false;
            }
            if i >= s.len() - k - 1 {
                break;
            }
        }
    }
    true
}

// https://www.acmicpc.net/problem/1972
// noinspection SpellCheckingInspection
// 놀라운 문자열
#[test]
fn test_solve1972() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ZGBG
X
EE
AAB
AABA
AABB
BCBABCC
*"
            .to_string(),
            want: "ZGBG is surprising.
X is surprising.
EE is surprising.
AAB is surprising.
AABA is surprising.
AABB is NOT surprising.
BCBABCC is NOT surprising."
                .to_string(),
        },
        TestData {
            s: "A
*"
            .to_string(),
            want: "A is surprising.".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1972(&mut reader, &mut writer);

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

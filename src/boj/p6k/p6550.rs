use crate::read_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6550(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    while reader.read_line(&mut line).is_ok() && !line.is_empty() {
        let (s, t) = read_values!(&line, String, String);

        let s_iter = s.char_indices();
        let mut t_iter = t.char_indices();

        let mut ans = true;

        for c in s_iter {
            for d in t_iter.by_ref() {
                if c.1 == d.1 {
                    break;
                }

                if d.0 == t.len() - 1 {
                    ans = false;
                    break;
                }
            }
        }

        if ans {
            writeln!(writer, "Yes").unwrap();
        } else {
            writeln!(writer, "No").unwrap();
        }
        line.clear();
    }
}

// https://www.acmicpc.net/problem/6550
// 부분 문자열
#[test]
fn test_solve6550() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "sequence subsequence".to_string(),
            want: "Yes
"
            .to_string(),
        },
        TestData {
            s: "sequence subsequence
person compression
VERDI vivaVittorioEmanueleReDiItalia
caseDoesMatter CaseDoesMatter"
                .to_string(),
            want: "Yes
No
Yes
No
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6550(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

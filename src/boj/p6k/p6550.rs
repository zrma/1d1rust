use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6550(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let mut line = String::new();
        let res = reader.read_line(&mut line);
        if res.is_err() {
            break;
        }

        if line.is_empty() {
            break;
        }

        let mut iter = line.split_whitespace();
        let s = iter.next().unwrap();
        let t = iter.next().unwrap();

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
    for (i, data) in vec![
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
        assert_eq!(got, data.want, "case {}", i);
    }
}

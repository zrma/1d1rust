use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4597(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let s = read_line(reader);
        if s == "#" {
            break;
        }

        let v: Vec<char> = s.chars().collect();
        let mut is_even = v.last().unwrap() == &'e';
        v.iter().take(v.len() - 1).for_each(|&c| {
            if c == '1' {
                is_even = !is_even;
            }
        });

        if is_even {
            writeln!(writer, "{}0", v[..v.len() - 1].iter().collect::<String>()).unwrap();
        } else {
            writeln!(writer, "{}1", v[..v.len() - 1].iter().collect::<String>()).unwrap();
        }
    }
}

// https://www.acmicpc.net/problem/4597
// 패리티
// noinspection SpellCheckingInspection
#[test]
fn test_solve4597() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "101e
010010o
1e
000e
110100101o
#"
            .to_string(),
            want: "1010
0100101
11
0000
1101001010
"
            .to_string(),
        },
        TestData {
            s: "101e
#"
            .to_string(),
            want: "1010
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4597(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

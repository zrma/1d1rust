use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11383(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, _) = read_values_as!(read_line(reader), usize, usize);

    let src = {
        let mut src = vec![];
        for _ in 0..n {
            src.push(read_line(reader));
        }
        src
    };

    // noinspection SpellCheckingInspection
    const OK: &str = "Eyfa";
    // noinspection SpellCheckingInspection
    const NG: &str = "Not Eyfa";

    for s in src.iter() {
        let arr = s.as_bytes();

        let cur = read_line(reader);
        let mut iter = cur.chars();

        for (j, c) in iter.by_ref().enumerate() {
            if c != arr[j / 2] as char {
                writeln!(writer, "{}", NG).unwrap();
                return;
            }
        }
    }
    writeln!(writer, "{}", OK).unwrap();
}

// https://www.acmicpc.net/problem/11383
// 뚊
// noinspection SpellCheckingInspection
#[test]
fn test_solve11383() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 5
ABCDE
AABBCCDDEE"
                .to_string(),
            want: "Eyfa".to_string(),
        },
        TestData {
            s: "1 5
ABCDE
AABBCCDDEF"
                .to_string(),
            want: "Not Eyfa".to_string(),
        },
        TestData {
            s: "2 2
AB
CD
AABB
CCDD"
                .to_string(),
            want: "Eyfa".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11383(&mut reader, &mut writer);

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

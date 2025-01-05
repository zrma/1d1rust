use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20124(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    type Entry = (String, i32);
    let mut top_entry: Option<Entry> = None;

    for _ in 0..n {
        let (name, score) = read_values_as!(read_line(reader), String, i32);

        top_entry = match top_entry {
            Some((top_name, top_score))
                if score > top_score || (score == top_score && name < top_name) =>
            {
                Some((name, score))
            }
            None => Some((name, score)),
            _ => top_entry,
        };
    }

    writeln!(writer, "{}", top_entry.unwrap().0).unwrap();
}

// https://www.acmicpc.net/problem/20124
// noinspection SpellCheckingInspection
// 모르고리즘 회장님 추천 받습니다
#[test]
fn test_solve20124() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
inseop 10
gukryeol 1
juno 11"
                .to_string(),
            want: "juno".to_string(),
        },
        TestData {
            s: "3
inseop 10
gukryeol 10
juno 10"
                .to_string(),
            want: "gukryeol".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve20124(&mut reader, &mut writer);

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

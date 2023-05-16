use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2954(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let res = replace_vowels(line);

    write!(writer, "{}", res).unwrap();
}

fn replace_vowels(input: String) -> String {
    let mut res = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        res.push(c);

        if is_vowel(c) {
            chars.next();
            chars.next();
        }
    }

    res
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

// https://www.acmicpc.net/problem/2954
// 창영이의 일기장
// noinspection SpellCheckingInspection
#[test]
fn test_solve2954() {
    struct TestData {
        s: String,
        want: String,
    }
    let tests = vec![
        TestData {
            s: "zepelepenapa papapripikapa".to_string(),
            want: "zelena paprika".to_string(),
        },
        TestData {
            s: "bapas jepe doposapadnapa opovapa kepemipijapa ".to_string(),
            want: "bas je dosadna ova kemija".to_string(),
        },
    ];

    for (i, data) in tests.iter().enumerate() {
        use std::io::Cursor;
        let mut reader = Cursor::new(data.s.clone());
        let mut writer = Cursor::new(Vec::new());
        solve2954(&mut reader, &mut writer);

        let res = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(res, data.want, "case {} failed", i);
    }
}

use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18409(reader: &mut impl BufRead, writer: &mut impl Write) {
    let _ = read_line(reader);
    let s = read_line(reader);

    let ans = s.chars().filter(|&c| "aeiou".contains(c)).count();
    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/18409
// 母音を数える (Counting Vowels)
#[test]
fn test_solve18409() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8
joiyosen"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "6
bitaro"
                .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18409(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

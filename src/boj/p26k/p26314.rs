use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve26314(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    for _ in 0..num_cases {
        let line = read_line(reader);
        let vowels = line.chars().filter(|&ch| "aeiou".contains(ch)).count();
        let consonants = line.len() - vowels;
        let ans: u8 = (vowels > consonants).into();
        writeln!(writer, "{}\n{}", line, ans).expect("write! failed");
    }
}

// https://www.acmicpc.net/problem/26314
// noinspection SpellCheckingInspection
// Vowel Count
#[test]
fn test_solve26314() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
ali
arup
travis
orooji"
                .to_string(),
            want: "ali
1
arup
0
travis
0
orooji
1
"
            .to_string(),
        },
        TestData {
            s: "2
abc
abe"
            .to_string(),
            want: "abc
0
abe
1
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve26314(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

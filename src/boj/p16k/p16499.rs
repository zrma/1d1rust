use crate::utils::io::{read_line, read_value};
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16499(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_words: usize = read_value(read_line(reader));
    let unique_words: HashSet<String> = (0..num_of_words)
        .map(|_| read_line(reader))
        .map(|s| {
            let mut chars = s.chars().collect::<Vec<char>>();
            chars.sort_unstable();
            chars.into_iter().collect::<_>()
        })
        .collect::<_>();

    write!(writer, "{}", unique_words.len()).expect("Failed to write");
}

// https://www.acmicpc.net/problem/16499
// noinspection SpellCheckingInspection
// 동일한 단어 그룹화하기
#[test]
fn test_solve16499() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
cat
dog
god
tca"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2
a
a"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve16499(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

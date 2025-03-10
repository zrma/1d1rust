use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
pub fn solve6321(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = read_line(reader);

    let n: i64 = line.trim().parse().unwrap();

    for i in 1..=n {
        line.clear();
        reader.read_line(&mut line).unwrap();

        let res: String = line
            .trim()
            .chars()
            .map(|c| if c == 'Z' { 'A' } else { (c as u8 + 1) as char })
            .collect();

        writeln!(writer, "String #{}", i).unwrap();
        writeln!(writer, "{}", res).unwrap();
        writeln!(writer).unwrap();
    }
}

// https://www.acmicpc.net/problem/6321
// IBM 빼기 1
// noinspection SpellCheckingInspection
#[test]
fn test_solve6321() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "2
HAL
SWERCZ"
            .to_string(),
        want: "String #1
IBM

String #2
TXFSDA

"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6321(&mut reader, &mut writer);

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

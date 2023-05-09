use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
pub fn solve6321(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = read_line(reader);

    let n = line.trim().parse::<usize>().unwrap();

    for i in 1..=n {
        line.clear();
        reader.read_line(&mut line).unwrap();

        let res: String = line
            .trim()
            .chars()
            .map(|c| if c == 'Z' { 'A' } else { (c as u8 + 1) as char })
            .collect();

        write!(writer, "String #{}\n{}\n\n", i, res).unwrap();
    }
}

// https://www.acmicpc.net/problem/6321
// IBM 빼기 1
//noinspection SpellCheckingInspection
#[test]
fn test_solve6321() {
    struct TestData {
        s: String,
        want: String,
    }
    for data in vec![TestData {
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
    }] {
        use std::io::Cursor;
        let mut reader = Cursor::new(data.s);
        let mut writer = Cursor::new(Vec::new());
        solve6321(&mut reader, &mut writer);

        let got = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(data.want, got);
    }
}

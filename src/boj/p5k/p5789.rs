use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
pub(crate) fn solve5789(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_line(reader).parse().unwrap();
    for _ in 0..t {
        let line_chars: Vec<char> = read_line(reader).chars().collect();
        let len = line_chars.len();
        let mid = len / 2;
        let left = line_chars[mid - 1];
        let right = line_chars[mid];
        let res = if left == right { "Do-it" } else { "Do-it-Not" };
        writeln!(writer, "{}", res).unwrap();
    }
}

// https://www.acmicpc.net/problem/5789
// 한다 안한다
#[test]
fn test_solve5789() {
    struct TestData {
        s: String,
        want: String,
    }
    for data in vec![TestData {
        s: "3
00100010
01010101
100001"
            .to_string(),
        want: "Do-it
Do-it-Not
Do-it
"
        .to_string(),
    }] {
        use std::io::Cursor;
        let mut cursor = Cursor::new(data.s);
        let mut output: Vec<u8> = Vec::new();
        solve5789(&mut cursor, &mut output);

        let got = String::from_utf8(output).unwrap();
        assert_eq!(data.want, got);
    }
}

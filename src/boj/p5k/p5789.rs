use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
pub(crate) fn solve5789(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_line(reader).parse().unwrap();
    for _ in 0..t {
        let s = read_line(reader);
        let line_chars = s.as_bytes();
        let len = line_chars.len();
        let mid = len / 2;
        let left = line_chars[mid - 1];
        let right = line_chars[mid];
        let res = if left == right { "Do-it" } else { "Do-it-Not" };
        writeln!(writer, "{}", res).expect("Failed to write");
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
    for (i, data) in [TestData {
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
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5789(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

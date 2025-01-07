use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2444(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_line(reader).parse().unwrap();

    for i in 1..=n {
        for _ in 0..(n - i) {
            write!(writer, " ").unwrap();
        }
        for _ in 0..(2 * i - 1) {
            write!(writer, "*").unwrap();
        }
        writeln!(writer).unwrap();
    }
    for i in (1..n).rev() {
        for _ in 0..(n - i) {
            write!(writer, " ").unwrap();
        }
        for _ in 0..(2 * i - 1) {
            write!(writer, "*").unwrap();
        }
        writeln!(writer).unwrap();
    }
}

// https://www.acmicpc.net/problem/2444
// 별 찍기 - 7
#[test]
fn test_solve2444() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "5".to_string(),
        want: "    *
   ***
  *****
 *******
*********
 *******
  *****
   ***
    *
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2444(&mut reader, &mut writer);

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

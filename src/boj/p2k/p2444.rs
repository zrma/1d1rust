use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2444(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<i64>().unwrap();

    for i in 1..=n {
        for _ in 0..(n - i) {
            write!(writer, " ").expect("Failed to write");
        }
        for _ in 0..(2 * i - 1) {
            write!(writer, "*").expect("Failed to write");
        }
        writeln!(writer).expect("Failed to write");
    }
    for i in (1..n).rev() {
        for _ in 0..(n - i) {
            write!(writer, " ").expect("Failed to write");
        }
        for _ in 0..(2 * i - 1) {
            write!(writer, "*").expect("Failed to write");
        }
        writeln!(writer).expect("Failed to write");
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

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

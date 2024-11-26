use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2522(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = (1..=n)
        .chain((1..n).rev())
        .map(|i| format!("{}{}", " ".repeat(n - i), "*".repeat(i)))
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/2522
// 별 찍기 - 12
#[test]
fn test_solve2522() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3".to_string(),
            want: "  *
 **
***
 **
  *"
            .to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "   *
  **
 ***
****
 ***
  **
   *"
            .to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "*".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2522(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

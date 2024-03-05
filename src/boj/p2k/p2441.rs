use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2441(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = (0..n)
        .rev()
        .map(|i| " ".repeat(n - i - 1) + &"*".repeat(i + 1))
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2441
// 별 찍기 - 4
#[test]
fn test_solve2441() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5".to_string(),
            want: "*****
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
        TestData {
            s: "2".to_string(),
            want: "**
 *"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2441(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

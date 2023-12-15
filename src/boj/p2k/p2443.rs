use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2443(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = (0..n)
        .map(|i| " ".repeat(i) + &"*".repeat(2 * (n - i) - 1))
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2443
// 별 찍기 - 6
#[test]
fn test_solve2443() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5".to_string(),
            want: "*********
 *******
  *****
   ***
    *"
            .to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "*".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "*****
 ***
  *"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2443(&mut reader, &mut writer);

        let result = String::from_utf8(writer).unwrap();
        assert_eq!(result, data.want, "failed at {}th case", i);
    }
}

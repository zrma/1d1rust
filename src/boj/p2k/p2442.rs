use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2442(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = (0..n)
        .map(|i| " ".repeat(n - i - 1) + &"*".repeat(2 * i + 1))
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2442
// 별 찍기 - 5
#[test]
fn test_solve2442() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5".to_string(),
            want: "    *
   ***
  *****
 *******
*********"
                .to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "*".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: " *
***"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2442(&mut reader, &mut writer);

        let result = String::from_utf8(writer).unwrap();
        assert_eq!(result, data.want, "failed at {}th case", i);
    }
}
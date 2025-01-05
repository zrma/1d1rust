use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10992(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    for i in 0..n {
        let padding = " ".repeat(n - i - 1);
        let stars = if i == 0 || i == n - 1 {
            "*".repeat(2 * i + 1)
        } else {
            format!("*{}*", " ".repeat(2 * i - 1))
        };
        writeln!(writer, "{}{}", padding, stars).unwrap();
    }
}

// https://www.acmicpc.net/problem/10992
// 별 찍기 - 17
#[test]
fn test_solve10992() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "*
"
            .to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: " *
***
"
            .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "  *
 * *
*****
"
            .to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "   *
  * *
 *   *
*******
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10992(&mut reader, &mut writer);

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

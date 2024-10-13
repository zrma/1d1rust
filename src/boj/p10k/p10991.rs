use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10991(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    for i in 0..n {
        let padding = " ".repeat(n - i - 1);
        let stars = "*".to_string() + &" *".repeat(i);
        writeln!(writer, "{}{}", padding, stars).expect("write! should work");
    }
}

// https://www.acmicpc.net/problem/10991
// 별 찍기 - 16
#[test]
fn test_solve10991() {
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
* *
"
            .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "  *
 * *
* * *
"
            .to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "   *
  * *
 * * *
* * * *
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10991(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10995(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    for i in 0..n {
        let padding = if i % 2 == 0 { "" } else { " " };
        writeln!(writer, "{}{}", padding, "* ".repeat(n).trim_end()).unwrap();
    }
}

// https://www.acmicpc.net/problem/10995
// 별 찍기 - 20
#[test]
fn test_solve10995() {
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
            want: "* *
 * *
"
            .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "* * *
 * * *
* * *
"
            .to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "* * * *
 * * * *
* * * *
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
        solve10995(&mut reader, &mut writer);

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

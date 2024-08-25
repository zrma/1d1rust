use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23794(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let border_line = "@".repeat(n + 2);
    let middle_line = format!("@{}@", " ".repeat(n));

    let mut ans = String::with_capacity((n + 2) * (n + 2));

    ans.push_str(&border_line);
    ans.push('\n');

    for _ in 0..n {
        ans.push_str(&middle_line);
        ans.push('\n');
    }

    ans.push_str(&border_line);

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/23794
// 골뱅이 찍기 - 정사각형
#[test]
fn test_solve23794() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "@@@
@ @
@@@"
            .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "@@@@@
@   @
@   @
@   @
@@@@@"
                .to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "@@@@@@@
@     @
@     @
@     @
@     @
@     @
@@@@@@@"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23794(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23811(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let long_line = "@".repeat(n * 5);
    let short_line = "@".repeat(n);

    let mut ans = String::with_capacity((n * 5 * 3) + n);

    append_pattern(&mut ans, n, &short_line, &long_line);

    writeln!(writer, "{}", ans).unwrap();
}

pub fn append_pattern(ans: &mut String, n: usize, even_line: &str, odd_line: &str) {
    for i in 0..5 {
        let line = if i % 2 == 0 { odd_line } else { even_line };
        for _ in 0..n {
            ans.push_str(line);
            ans.push('\n');
        }
    }
}

// https://www.acmicpc.net/problem/23811
// 골뱅이 찍기 - ㅌ
#[test]
fn test_solve23811() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "@@@@@
@
@@@@@
@
@@@@@"
                .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@
@@@
@@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@
@@@
@@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23811(&mut reader, &mut writer);

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

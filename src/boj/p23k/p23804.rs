use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23804(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let border_line = "@".repeat(n * 5);
    let middle_line = "@".repeat(n);

    let mut ans = String::with_capacity((n * 5 * 2 + n * 3) * (n + 1));

    for _ in 0..n {
        ans.push_str(&border_line);
        ans.push('\n');
    }
    for _ in 0..(n * 3) {
        ans.push_str(&middle_line);
        ans.push('\n');
    }
    for _ in 0..n {
        ans.push_str(&border_line);
        ans.push('\n');
    }

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/23804
// 골뱅이 찍기 - ㄷ
#[test]
fn test_solve23804() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "@@@@@
@
@
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
@@@
@@@
@@@
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
        solve23804(&mut reader, &mut writer);

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

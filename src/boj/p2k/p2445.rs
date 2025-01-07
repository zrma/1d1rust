use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2448(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = (1..=n)
        .chain((1..n).rev())
        .map(|i| create_line(i, n))
        .collect::<Vec<_>>()
        .join("\n");

    writeln!(writer, "{}", ans).unwrap();
}

fn create_line(i: usize, n: usize) -> String {
    format!(
        "{}{}{}",
        "*".repeat(i),
        " ".repeat(2 * (n - i)),
        "*".repeat(i)
    )
}

// https://www.acmicpc.net/problem/2448
// 별 찍기 - 11
#[test]
fn test_solve2448() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5".to_string(),
            want: "*        *
**      **
***    ***
****  ****
**********
****  ****
***    ***
**      **
*        *"
                .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "*    *
**  **
******
**  **
*    *"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2448(&mut reader, &mut writer);

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

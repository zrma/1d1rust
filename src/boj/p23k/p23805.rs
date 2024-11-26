use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23805(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut top_line = String::with_capacity(5 * n);
    top_line.push_str(&"@".repeat(3 * n));
    top_line.push_str(&" ".repeat(n));
    top_line.push_str(&"@".repeat(n));

    let mut middle_line = String::with_capacity(5 * n);
    middle_line.push_str(&"@".repeat(n));
    middle_line.push_str(&" ".repeat(n));
    middle_line.push_str(&"@".repeat(n));
    middle_line.push_str(&" ".repeat(n));
    middle_line.push_str(&"@".repeat(n));

    let mut bottom_line = String::with_capacity(5 * n);
    bottom_line.push_str(&"@".repeat(n));
    bottom_line.push_str(&" ".repeat(n));
    bottom_line.push_str(&"@".repeat(3 * n));

    let mut ans = String::with_capacity(5 * n * 5 * n);

    for _ in 0..n {
        ans.push_str(&top_line);
        ans.push('\n');
    }

    for _ in 0..(3 * n) {
        ans.push_str(&middle_line);
        ans.push('\n');
    }

    for _ in 0..n {
        ans.push_str(&bottom_line);
        ans.push('\n');
    }

    ans.pop(); // remove last '\n'

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/23805
// 골뱅이 찍기 - 돌아간 ㄹ
#[test]
fn test_solve23805() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "@@@ @
@ @ @
@ @ @
@ @ @
@ @@@"
                .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "@@@@@@@@@   @@@
@@@@@@@@@   @@@
@@@@@@@@@   @@@
@@@   @@@   @@@
@@@   @@@   @@@
@@@   @@@   @@@
@@@   @@@   @@@
@@@   @@@   @@@
@@@   @@@   @@@
@@@   @@@   @@@
@@@   @@@   @@@
@@@   @@@   @@@
@@@   @@@@@@@@@
@@@   @@@@@@@@@
@@@   @@@@@@@@@"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23805(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

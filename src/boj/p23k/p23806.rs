use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23806(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let border_line = "@".repeat(n * 5);
    let middle_line = format!("{}{}{}", "@".repeat(n), " ".repeat(n * 3), "@".repeat(n));

    let mut ans = String::with_capacity(5 * n * 5 * n);

    for _ in 0..n {
        ans.push_str(&border_line);
        ans.push('\n');
    }

    for _ in 0..(3 * n) {
        ans.push_str(&middle_line);
        ans.push('\n');
    }

    for _ in 0..n {
        ans.push_str(&border_line);
        ans.push('\n');
    }

    ans.pop(); // remove last '\n'

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/23806
// 골뱅이 찍기 - ㅁ
#[test]
fn test_solve23806() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "@@@@@
@   @
@   @
@   @
@@@@@"
                .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@         @@@
@@@         @@@
@@@         @@@
@@@         @@@
@@@         @@@
@@@         @@@
@@@         @@@
@@@         @@@
@@@         @@@
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
        solve23806(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

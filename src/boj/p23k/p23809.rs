use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23809(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut ans = String::with_capacity(5 * n * 5 * n);

    let pattern = |i: usize| {
        let mut line = String::with_capacity(5 * n);
        line.push_str(&"@".repeat(n));
        line.push_str(&" ".repeat(n * i));
        line.push_str(&"@".repeat(n));
        line
    };

    for i in (2..4).rev() {
        let line = pattern(i);
        for _ in 0..n {
            ans.push_str(&line);
            ans.push('\n');
        }
    }

    let middle_line = "@".repeat(3 * n);
    for _ in 0..n {
        ans.push_str(&middle_line);
        ans.push('\n');
    }

    for i in 2..4 {
        let line = pattern(i);
        for _ in 0..n {
            ans.push_str(&line);
            ans.push('\n');
        }
    }

    ans.pop(); // remove the last '\n'

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/23809
// 골뱅이 찍기 - 돌아간 ㅈ
#[test]
fn test_solve23809() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "@   @
@  @
@@@
@  @
@   @"
                .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "@@@         @@@
@@@         @@@
@@@         @@@
@@@      @@@
@@@      @@@
@@@      @@@
@@@@@@@@@
@@@@@@@@@
@@@@@@@@@
@@@      @@@
@@@      @@@
@@@      @@@
@@@         @@@
@@@         @@@
@@@         @@@"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23809(&mut reader, &mut writer);

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

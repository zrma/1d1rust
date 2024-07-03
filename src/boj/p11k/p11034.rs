use crate::read_values_as;
use std::cmp::max;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11034(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut answers = vec![];
    let mut line = String::new();

    while reader.read_line(&mut line).unwrap_or(0) > 0 {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }
        let (a, b, c) = read_values_as!(trimmed_line, i64, i64, i64);
        let ans = max(b - a, c - b) - 1;
        answers.push(ans.to_string());
        line.clear();
    }

    write!(writer, "{}", answers.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/11034
// 캥거루 세마리2
#[test]
fn test_solve11034() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 3 5
3 5 9"
                .to_string(),
            want: "1
3"
            .to_string(),
        },
        TestData {
            s: "0 5 9".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "1 5 9".to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11034(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8871(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));

    // n+1개의 라운드에서 각각 최소 2번, 최대 3번의 시도가 필요
    let min = (n + 1) * 2;
    let max = (n + 1) * 3;

    writeln!(writer, "{} {}", min, max).unwrap();
}

// https://www.acmicpc.net/problem/8871
// Zadanie próbne 2
#[test]
fn test_solve8871() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "5".to_string(),
            want: "12 18".to_string(),
        },
        TestCase {
            s: "0".to_string(),
            want: "2 3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8871(&mut reader, &mut writer);

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

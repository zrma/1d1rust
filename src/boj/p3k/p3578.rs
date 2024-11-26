use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3578(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = match n {
        0 => "1".to_string(),
        1 => "0".to_string(),
        _ => format!("{}{}", if n % 2 == 0 { "" } else { "4" }, "8".repeat(n / 2)),
    };

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/3578
// Holes
#[test]
fn test_solve3578() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "48".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "88".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "488".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "888".to_string(),
        },
        TestData {
            s: "15".to_string(),
            want: "48888888".to_string(),
        },
        TestData {
            s: "16".to_string(),
            want: "88888888".to_string(),
        },
        TestData {
            s: "70".to_string(),
            want: "88888888888888888888888888888888888".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3578(&mut reader, &mut writer);

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

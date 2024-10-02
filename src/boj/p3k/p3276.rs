use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3276(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let (r_ans, c_ans) = (1..)
        .take_while(|&r| r * r <= n)
        .map(|r| (r, (n + r - 1) / r))
        .min_by_key(|&(r, c)| r + c)
        .unwrap_or((1, n));

    write!(writer, "{} {}", r_ans, c_ans).expect("Failed to write output");
}

// https://www.acmicpc.net/problem/3276
// ICONS
#[test]
fn test_solve3276() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2".to_string(),
            want: "1 2".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "2 3".to_string(),
        },
        TestData {
            s: "14".to_string(),
            want: "3 5".to_string(),
        },
        TestData {
            s: "9".to_string(),
            want: "3 3".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "2 4".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "2 4".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "2 3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3276(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

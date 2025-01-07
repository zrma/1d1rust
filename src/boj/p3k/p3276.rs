use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3276(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let (r_ans, c_ans) = (1..)
        .take_while(|&r| r * r <= n)
        .map(|r| (r, n.div_ceil(r)))
        .min_by_key(|&(r, c)| r + c)
        .unwrap_or((1, n));

    writeln!(writer, "{} {}", r_ans, c_ans).unwrap();
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

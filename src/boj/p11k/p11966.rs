use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11966(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: u32 = read_value(read_line(reader));
    let is_power_of_two = (n != 0) && (n & (n - 1) == 0);
    let ans = if is_power_of_two { 1 } else { 0 };

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/11966
// 2의 제곱인가?
#[test]
fn test_solve11966() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "9".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "10".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "11".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "12".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "9".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "16".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "17".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11966(&mut reader, &mut writer);

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

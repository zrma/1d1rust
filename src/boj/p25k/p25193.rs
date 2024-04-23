use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25193(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let chars = read_line(reader);

    let c_count = chars.chars().filter(|&c| c == 'C').count();
    let k_count = n - c_count;
    let group_count = k_count + 1;

    let max_c_per_group = (c_count + group_count - 1) / group_count;
    write!(writer, "{}", max_c_per_group).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25193
// noinspection SpellCheckingInspection
// 곰곰이의 식단 관리
#[test]
fn test_solve25193() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
CCHCCCK"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "4
CCCC"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "5
ACCCC"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "7
CKCKCKC"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve25193(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

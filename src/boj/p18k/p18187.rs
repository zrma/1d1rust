use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18187(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));
    let mut ans = 2;
    let mut diff = 2;

    for i in 1..n {
        ans += diff;
        if i % 3 != 2 {
            diff += 1;
        }
    }

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/18187
// 평면 분할
#[test]
fn test_solve18187() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "14".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "19".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "24".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "30".to_string(),
        },
        TestData {
            s: "9".to_string(),
            want: "37".to_string(),
        },
        TestData {
            s: "10".to_string(),
            want: "44".to_string(),
        },
        TestData {
            s: "11".to_string(),
            want: "52".to_string(),
        },
        TestData {
            s: "12".to_string(),
            want: "61".to_string(),
        },
        TestData {
            s: "13".to_string(),
            want: "70".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18187(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

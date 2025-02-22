use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15727(reader: &mut impl BufRead, writer: &mut impl Write) {
    let l: i32 = read_value(read_line(reader));
    let result = (l + 4) / 5;
    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/15727
// 조별과제를 하려는데 조장이 사라졌다
#[test]
fn test_solve15727() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "12".to_string(),
            want: "3".to_string(),
        },
        TestCase {
            s: "5".to_string(),
            want: "1".to_string(),
        },
        TestCase {
            s: "14".to_string(),
            want: "3".to_string(),
        },
        TestCase {
            s: "1".to_string(),
            want: "1".to_string(),
        },
        TestCase {
            s: "13".to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15727(&mut reader, &mut writer);

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

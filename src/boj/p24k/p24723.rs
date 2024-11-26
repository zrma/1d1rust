use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24723(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();
    let mut ans = 1;
    for _ in 0..n {
        ans *= 2;
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/24723
// 녹색거탑
#[test]
fn test_solve24723() {
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
            want: "8".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "16".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve24723(&mut reader, &mut writer);

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

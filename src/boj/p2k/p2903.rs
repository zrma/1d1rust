use std::io::{BufRead, Write};

use crate::utils::io::read_line;

#[allow(dead_code)]
fn solve2903(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<u64>().unwrap();

    let mut res = 2;
    for _ in 0..n {
        res = res * 2 - 1;
    }

    let res = res * res;
    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/2903
// 중앙 이동 알고리즘
#[test]
fn test_solve2903() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "1".to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "25".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "81".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "289".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "1089".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2903(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

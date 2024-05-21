use crate::utils::io::read_n_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2959(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut v = read_n_values::<i32>(reader, 4);
    v.sort_unstable();
    let ans = v[0] * v[2];
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2959
// 거북이
#[test]
fn test_solve2959() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2 3 4".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "4 4 3 4".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "1 1 1 1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 2 1 1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 2 1 2".to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2959(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

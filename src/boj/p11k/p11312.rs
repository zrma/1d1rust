use crate::boj::p11k::p11320::count_cover_tris;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11312(reader: &mut impl BufRead, writer: &mut impl Write) {
    count_cover_tris(reader, writer);
}

// https://www.acmicpc.net/problem/11312
// 삼각 무늬 - 2
#[test]
fn test_solve11312() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
2 1
3 3
4 1"
            .to_string(),
            want: "4
1
16"
            .to_string(),
        },
        TestData {
            s: "2
3 1
1 1"
            .to_string(),
            want: "9
1"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11312(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

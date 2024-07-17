use crate::utils::io::read_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14909(reader: &mut impl BufRead, writer: &mut impl Write) {
    let numbers: Vec<i32> = read_values(reader);
    let ans = numbers.iter().filter(|&&num| num > 0).count();

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/14909
// 양수 개수 세기
#[test]
fn test_solve14909() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "3 9 11 32 8 2 6".to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "-2 0 21 3 8 17 32 -8 7 0".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "0".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14909(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

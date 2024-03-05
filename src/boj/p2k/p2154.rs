use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2154(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let sequence = generate_sequence(n);
    let ans = find_position(&sequence, n);
    write!(writer, "{}", ans).unwrap();
}

fn generate_sequence(n: usize) -> String {
    (1..=n).map(|i| i.to_string()).collect()
}

fn find_position(sequence: &str, n: usize) -> usize {
    sequence.find(&n.to_string()).unwrap() + 1
}

// https://www.acmicpc.net/problem/2154
// 수 이어 쓰기 3
#[test]
fn test_solve2154() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "15".to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "151".to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "34".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "142".to_string(),
            want: "73".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2154(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}

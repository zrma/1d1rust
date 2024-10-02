use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9243(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader);
    let s = read_line(reader);
    let t = read_line(reader);

    let div = n.parse::<usize>().unwrap() % 2;
    let ans = s
        .chars()
        .zip(t.chars())
        .all(|(c1, c2)| (c1 == c2) ^ (div != 0));

    write!(
        writer,
        "{}",
        if ans {
            "Deletion succeeded"
        } else {
            "Deletion failed"
        }
    )
    .unwrap();
}

// https://www.acmicpc.net/problem/9243
// 파일 완전 삭제
#[test]
fn test_solve9243() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
10001110101000001111010100001110
01110001010111110000101011110001"
                .to_string(),
            want: "Deletion succeeded".to_string(),
        },
        TestData {
            s: "2
10001110101000001111010100001110
10001110101000001111010100001110"
                .to_string(),
            want: "Deletion succeeded".to_string(),
        },
        TestData {
            s: "3
10001110101000001111010100001110
01110001010111110000101011110001"
                .to_string(),
            want: "Deletion succeeded".to_string(),
        },
        TestData {
            s: "3
10001110101000001111010100001110
01110001010111110000101011110000"
                .to_string(),
            want: "Deletion failed".to_string(),
        },
        TestData {
            s: "20
0001100011001010
0001000011000100"
                .to_string(),
            want: "Deletion failed".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9243(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
